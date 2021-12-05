use super::DayTask;
use anyhow::Context;
use anyhow::Result as AResult;

pub struct Day3;

impl DayTask for Day3 {
    type Out = (u64, u64);
    const NAME: &'static str = "day3";

    fn run_a(lines: Vec<String>) -> AResult<Self::Out> {
        let cutoff = lines.len();
        let len = lines.get(0).context("There was no input")?.len();

        let counts = lines
            .iter()
            .map(|l| l.chars().map(|c| if c == '1' { 1 } else { 0 }))
            .fold(
                {
                    let mut it = Vec::with_capacity(len);
                    it.resize(len, 0);
                    it
                },
                |mut a, b| {
                    b.enumerate().for_each(|(idx, it)| a[idx] += it);
                    a
                },
            );

        let gamma = counts.iter().enumerate().fold(0, |it, (idx, count)| {
            if *count > cutoff / 2 {
                println!("{}", len - idx);
                it | 1 << (len - idx - 1)
            } else {
                it
            }
        });

        let mask = (0..len).into_iter().fold(0, |acc, _| (acc << 1) | 1);

        let epsilon = !gamma & mask;

        Ok((gamma, epsilon))
    }

    fn run_b(lines: Vec<String>) -> AResult<Self::Out> {
        let cutoff = lines.len();
        let len = lines.get(0).context("There was no input")?.len();

        // TODO(josiah) the counts have to change for these on each iteration of the map below, so
        // the method of only calculating indicies once will not work.

        let counts = lines
            .iter()
            .map(|l| l.chars().map(|c| if c == '1' { 1 } else { 0 }))
            .fold(
                {
                    let mut it = Vec::with_capacity(len);
                    it.resize(len, 0);
                    it
                },
                |mut a, b| {
                    b.enumerate().for_each(|(idx, it)| a[idx] += it);
                    a
                },
            );

        let (oxygen, co2, _, _) = lines
            .iter()
            .map(|l| {
                (
                    l,
                    l.chars().enumerate().find_map(|(idx, c)| {
                        let v = if counts[idx] > cutoff / 2 { '1' } else { '0' };
                        if c != v {
                            Some(idx)
                        } else {
                            None
                        }
                    }),
                    l.chars().enumerate().find_map(|(idx, c)| {
                        let v = if counts[idx] < cutoff / 2 { '1' } else { '0' };
                        if c != v {
                            Some(idx)
                        } else {
                            None
                        }
                    }),
                )
            })
            .fold(
                (String::new(), String::new(), 0, 0),
                |(oxy, co2, oxi, coi), (l, oxygen_b, co2_b)| {
                    let (oxy, oxi) = {
                        match oxygen_b {
                            Some(oxygen_b) if oxygen_b > oxi => (l.clone(), oxygen_b),
                            _ => (oxy, oxi),
                        }
                    };

                    let (co2, coi) = {
                        match co2_b {
                            Some(co2_b) if co2_b > coi => (l.clone(), co2_b),
                            _ => (co2, coi),
                        }
                    };

                    (oxy, co2, oxi, coi)
                },
            );

        fn to_num(val: String) -> u64 {
            val.chars().into_iter().fold(
                0,
                |acc, char| if char == '1' { acc << 1 | 1 } else { acc << 1 },
            )
        }

        Ok((to_num(oxygen), to_num(co2)))
    }
}
