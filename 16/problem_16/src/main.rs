use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Range;
use regex::Regex;
use lazy_static::lazy_static;

type Rules = HashMap<String, (Range<u32>, Range<u32>)>;
type Ticket = Vec<u32>;

fn get_rules(input :&String) -> Rules
{
    lazy_static!
    {
        static ref RULES_REGEX :Regex = 
            Regex::new(r"([\w ]+): ([0-9\-]+) or ([0-9\-]+)").unwrap();
    }

    RULES_REGEX.captures_iter(input)
                .filter_map(|caps|
                            {
                                let rule_name = caps.get(1)?.as_str().to_owned();
                                let range1 = extract_range(caps.get(2)?.as_str());
                                let range2 = extract_range(caps.get(3)?.as_str());
                                Some((rule_name, (range1, range2)))
                            })
                .collect()
}

fn extract_range(input :&str) -> Range<u32>
{
    let range :Vec<u32> = input.split("-")
                               .map(|num| num.parse::<u32>().unwrap())
                               .collect();
    range[0]..range[1] + 1
}

fn get_your_ticket(input :&String) -> Ticket
{
    lazy_static!
    {
        static ref YOUR_TICKET_REGEX :Regex = 
            Regex::new(r"your ticket:\n([\d,]+)").unwrap();
    }
    
    let ticket_captures = YOUR_TICKET_REGEX.captures(input).unwrap();
    ticket_captures.get(1).unwrap()
                          .as_str()
                          .split(",")
                          .filter_map(|num| num.parse::<u32>().ok())
                          .collect()
}

fn get_nearby_tickets(input :&String) -> Vec<Ticket>
{
    lazy_static!
    {
        static ref NEARBY_TICKETS_REGEX :Regex = 
            Regex::new(r"nearby tickets:\n([\d,\n]+)").unwrap();
    }
    
    let nearby_tickets = NEARBY_TICKETS_REGEX.captures(input).unwrap();
    nearby_tickets.get(1).unwrap()
                         .as_str()
                         .split_whitespace()
                         .map(|ticket| 
                            ticket.split(",")
                                  .filter_map(|num| num.parse::<u32>().ok())
                                  .collect()
                              )
                         .collect()
    
}

fn get_invalid_values<'a>(ticket :&'a Ticket, rules :&Rules) -> Vec<&'a u32>
{
    ticket.iter().filter(|value|
                      {
                        !rules.values()
                               .any(|ranges|
                                   {
                                     ranges.0.contains(&value) || 
                                         ranges.1.contains(&value)
                                   })
                      }).collect()
}

fn part1(nearby_tickets :&Vec<Ticket>, rules :&Rules)
{
    let invalid_values :Vec<&u32> = nearby_tickets.iter()
                                       .map(|ticket| 
                                            get_invalid_values(&ticket, &rules))
                                       .flatten()
                                       .collect();
    println!("Ticket scanning error rate: {}", invalid_values.into_iter().sum::<u32>());
}

fn all_in_ranges(values :&Vec<&u32>, ranges :&(Range<u32>, Range<u32>)) -> bool
{
    values.iter().all(|value| ranges.0.contains(&value) || ranges.1.contains(&value))
}

fn part2(your_ticket :&Ticket, nearby_tickets :&Vec<Ticket>, rules :&Rules)
{
    let num_fields = your_ticket.len();
    let valid_tickets :Vec<_> = nearby_tickets
                                        .iter()
                                        .filter(|ticket| 
                                                get_invalid_values(&ticket, &rules).len() == 0)
                                        .collect();


    let ticket_field_values :Vec<_> = 
            (0..num_fields).map(|idx| valid_tickets
                                        .iter()
                                        .map(|ticket| &ticket[idx])
                                        .collect()
                            )
                            .collect();

    let mut possible_field_idxs :HashMap<_, _> = 
            rules.iter().map(|(name, ranges)|
                             {
                                let possible_idxs :HashSet<usize> = ticket_field_values
                                                        .iter()
                                                        .enumerate()
                                                        .filter_map(|(field_idx, values)|
                                                            if all_in_ranges(values, &ranges)
                                                            { Some(field_idx) }
                                                            else
                                                            { None }).collect();
                                (name, possible_idxs)
                             }).collect();
    
    let fields_sorted :Vec<&String> = 
    {
        let mut pfi_as_vec :Vec<(_,_)> = possible_field_idxs.iter().collect();
        pfi_as_vec.sort_by_key(|(_, possible_idxs)| possible_idxs.len());
        pfi_as_vec.into_iter().map(|(field, _)| *field).collect()
    };

    let field_idxs :HashMap<_, _> 
        = fields_sorted
            .iter()
            .map(|field|
                {
                    assert_eq!(possible_field_idxs[field].len(), 1);
                    let resolved_field_index = possible_field_idxs[field]
                                                    .iter()
                                                    .next()
                                                    .unwrap().clone();

                    possible_field_idxs
                        .values_mut()
                        .for_each(|idxs| 
                                  {
                                      idxs.remove(&resolved_field_index);
                                  });
        (field, resolved_field_index)
    }).collect();

    println!("Possible field indexes: {:#?}", field_idxs);               

    let answer = field_idxs.iter() 
                  .filter(|(field, _)| field.starts_with("departure"))
                  .map(|(_, idx)| your_ticket[*idx] as u64).product::<u64>();
    
    println!("Part2 answer: {}", answer); 
}

fn main() 
{
    let args :Vec<String> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1]).expect("Could not open file");

    let rules = get_rules(&input);
    let your_ticket = get_your_ticket(&input);
    let nearby_tickets = get_nearby_tickets(&input);
    
    part1(&nearby_tickets, &rules);
    part2(&your_ticket, &nearby_tickets, &rules);
}
