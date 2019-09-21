pub fn build_proverb_imperative(list: &[&str]) -> String {
    let mut proverb = String::new();
    if !list.is_empty() {
        for index in 0..(list.len() - 1) {
            proverb = format!(
                "{}For want of a {} the {} was lost.\n",
                proverb,
                list[index],
                list[index + 1]
            );
        }
        proverb = format!("{}And all for the want of a {}.", proverb, list[0])
    }
    proverb
}

pub fn build_proverb_oo(list: &[&str]) -> String {
    let proverb = proverb_builder::ProverbBuilder::new(list);
    proverb.get_proverb()
}

pub fn build_proverb_old(list: &[&str]) -> String {
    let mut iter_list = list.iter().peekable();
    let mut proverb = String::new();

    while let Some(&word) = iter_list.next() {
        if let Some(&&next_word) = iter_list.peek() {
            proverb = format!(
                "{}For want of a {} the {} was lost.\n",
                proverb, word, next_word
            );
        } else {
            proverb = format!(
                "{}And all for the want of a {}.",
                proverb,
                list.first().unwrap()
            )
        }
    }
    proverb
}

pub fn build_proverb_func(list: &[&str]) -> String {
    match list {
        [] => String::new(),
        _ => list
            .windows(2)
            .map(|words| format!("For want of a {} the {} was lost.", words[0], words[1]))
            .chain(std::iter::once(format!(
                "And all for the want of a {}.",
                list[0]
            )))
            .collect::<Vec<String>>()
            .join("\n"),
    }
}

pub mod proverb_builder {
    pub struct ProverbBuilder {
        list_of_words: Vec<String>,
    }

    impl ProverbBuilder {
        pub fn new(list_to_build_from: &[&str]) -> ProverbBuilder {
            if list_to_build_from.is_empty() {
                ProverbBuilder {
                    list_of_words: Vec::new(),
                }
            } else {
                let mut vec_of_words: Vec<String> = Vec::new();

                for &word in list_to_build_from {
                    vec_of_words.push(String::from(word));
                }

                ProverbBuilder {
                    list_of_words: vec_of_words,
                }
            }
        }
        pub fn get_words(self) -> Vec<String> {
            self.list_of_words
        }

        pub fn get_proverb(self) -> String {
            let mut proverb: Vec<String> = Vec::new();

            if self.list_of_words.is_empty() {
                String::new()
            } else {
                for index in 0..(self.list_of_words.len() - 1) {
                    proverb.push(format!(
                        "For want of a {} the {} was lost.",
                        self.list_of_words.get(index).unwrap(),
                        self.list_of_words.get(index + 1).unwrap()
                    ));
                }
                proverb.push(format!(
                    "And all for the want of a {}.",
                    self.list_of_words.first().unwrap()
                ));
                proverb.join("\n")
            }
        }
    }
}
