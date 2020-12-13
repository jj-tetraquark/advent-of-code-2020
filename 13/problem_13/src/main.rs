fn main() 
{
    let args :Vec<String> = std::env::args().collect();
    let data :Vec<String> = std::fs::read_to_string(&args[1])
                                        .expect("Could not read from file")
                                        .split_whitespace()
                                        .map(str::to_owned)
                                        .collect();

    let time = data[0].parse::<u32>().unwrap();
    let services : Vec<&str> = data[1].split(',').collect();

    let (bus, wait) = services.iter().filter_map(|value|
                                                  {
                                                      let service = value.parse::<u32>().ok()?;
                                                      let arrival_time = (time/service + 1) * service;
                                                      let wait_time = arrival_time - time;
                                                      Some((service, wait_time))
                                                  })
                                     .min_by_key(|(_, wait_time)| *wait_time).unwrap();

    println!("Waiting {} minutes for the {} bus", wait, bus);
    println!("{}", wait * bus);

    // part 2
    let service_with_offsets :Vec<(u64, u64)> = services.iter()
                                                        .enumerate()
                                                        .filter_map(|(idx, value)|
                                                                    {
                                                                        let service = value.parse::<u64>().ok()?;
                                                                        Some((service, idx as u64))
                                                                    })
                                                        .collect();

    println!("Services with offsets: {:?}", service_with_offsets);

    let mut step = service_with_offsets[0].0;
    let mut earliest_time = service_with_offsets[0].1 + step;

    service_with_offsets.iter().skip(1).for_each(|(service, offset)|
                                                 {
                                                    while (earliest_time + offset) % service != 0
                                                    {
                                                        earliest_time += step;
                                                    }
                                                    let factorised_time = earliest_time;
                                                    earliest_time += step;
                                                    while (earliest_time + offset) % service != 0
                                                    {
                                                        earliest_time += step;
                                                    }
                                                    step = earliest_time - factorised_time;
                                                 });

    println!("Earliest time: {}", earliest_time - step);

}
