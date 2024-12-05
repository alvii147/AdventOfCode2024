use std::fs;

struct WordSearch {
    len: (usize, usize),
    words: Vec<Vec<char>>,
}

impl WordSearch {
    fn new(words_str: String) -> Self {
        let len: (usize, usize);
        let words: Vec<Vec<char>> = words_str
            .split_whitespace()
            .map(|s| s.chars().collect())
            .collect();

        if words.is_empty() {
            len = (0, 0);
        } else {
            let n_rows: usize = words.len();
            let n_cols: usize = words
                .get(0)
                .expect("Index should not have been out of bounds")
                .len();
            len = (n_rows, n_cols);
        }

        return Self { len, words };
    }

    fn get(&self, i: usize, j: usize) -> &char {
        return self
            .words
            .get(i)
            .expect("i index out of bounds")
            .get(j)
            .expect("j index out of range");
    }

    fn is_xmas(&self, i: usize, j: usize) -> u32 {
        let (n_rows, n_cols): (usize, usize) = self.len;
        let mut matches: u32 = 0;

        if *self.get(i, j) != 'X' {
            return 0;
        }

        if j + 3 < n_cols
            && *self.get(i, j + 1) == 'M'
            && *self.get(i, j + 2) == 'A'
            && *self.get(i, j + 3) == 'S'
        {
            matches += 1;
        }

        if j >= 3
            && *self.get(i, j - 1) == 'M'
            && *self.get(i, j - 2) == 'A'
            && *self.get(i, j - 3) == 'S'
        {
            matches += 1;
        }

        if i + 3 < n_rows
            && *self.get(i + 1, j) == 'M'
            && *self.get(i + 2, j) == 'A'
            && *self.get(i + 3, j) == 'S'
        {
            matches += 1;
        }

        if i >= 3
            && *self.get(i - 1, j) == 'M'
            && *self.get(i - 2, j) == 'A'
            && *self.get(i - 3, j) == 'S'
        {
            matches += 1;
        }

        if i + 3 < n_rows
            && j + 3 < n_cols
            && *self.get(i + 1, j + 1) == 'M'
            && *self.get(i + 2, j + 2) == 'A'
            && *self.get(i + 3, j + 3) == 'S'
        {
            matches += 1;
        }

        if i >= 3
            && j >= 3
            && *self.get(i - 1, j - 1) == 'M'
            && *self.get(i - 2, j - 2) == 'A'
            && *self.get(i - 3, j - 3) == 'S'
        {
            matches += 1;
        }

        if i + 3 < n_rows
            && j >= 3
            && *self.get(i + 1, j - 1) == 'M'
            && *self.get(i + 2, j - 2) == 'A'
            && *self.get(i + 3, j - 3) == 'S'
        {
            matches += 1;
        }

        if i >= 3
            && j + 3 < n_cols
            && *self.get(i - 1, j + 1) == 'M'
            && *self.get(i - 2, j + 2) == 'A'
            && *self.get(i - 3, j + 3) == 'S'
        {
            matches += 1;
        }

        return matches;
    }

    fn is_x_mas(&self, i: usize, j: usize) -> bool {
        let (n_rows, n_cols): (usize, usize) = self.len;

        if i < 1 || j < 1 || i > n_rows - 2 || j > n_cols - 2 {
            return false;
        }

        if *self.get(i, j) != 'A' {
            return false;
        }

        let top_left: &char = self.get(i - 1, j - 1);
        let top_right: &char = self.get(i - 1, j + 1);
        let bottom_left: &char = self.get(i + 1, j - 1);
        let bottom_right: &char = self.get(i + 1, j + 1);

        if *top_left != 'S' && *top_left != 'M' {
            return false;
        }

        if *top_right != 'S' && *top_right != 'M' {
            return false;
        }

        if *bottom_left != 'S' && *bottom_left != 'M' {
            return false;
        }

        if *bottom_right != 'S' && *bottom_right != 'M' {
            return false;
        }

        if *top_left == *bottom_right {
            return false;
        }

        if *top_right == *bottom_left {
            return false;
        }

        return true;
    }
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Should have been able to read file");
}

fn main() {
    let file_path: &str = "../words.txt";
    let file_contents: String = read_file(file_path);
    let word_search: WordSearch = WordSearch::new(file_contents);

    let (n_rows, n_cols): (usize, usize) = word_search.len;
    let mut xmas_count: u32 = 0;
    let mut x_mas_count: u32 = 0;
    for i in 0..n_rows {
        for j in 0..n_cols {
            xmas_count += word_search.is_xmas(i, j);
            if word_search.is_x_mas(i, j) {
                x_mas_count += 1;
            }
        }
    }

    println!("Part 1: {xmas_count}");
    println!("Part 2: {x_mas_count}");
}
