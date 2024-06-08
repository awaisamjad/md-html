// pub struct KeysWithSpace {
//     headers: Headers,
//     list: List,
// }

pub struct KeysWithSpace {
    pub h1: String,
    pub h2: String,
    pub h3: String,
    pub h4: String,
    pub h5: String,
    pub h6: String,
    // pub ordered_list: String, //TODO Needs to be handled separately as the number can change
    pub unordered_list: String,
}

pub struct Headers {
    pub h1: String,
    pub h2: String,
    pub h3: String,
    pub h4: String,
    pub h5: String,
    pub h6: String,
}

pub struct List {
    ordered: String,
    unordered: String,
}

struct Style {
    strikethrough: String,
    bold: String,
    italic: String,
    highlight: String,
}

struct Code {
    single: String,
    multi_line: String,
}
