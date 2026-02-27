//! This module takes care of `csv` parsing of the user and products file.

/// Represents each element inside the `csv`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Record<'r> {
    /// `DATA_COMPRA`
    pub timestamp: u32,

    /// `COD_CLIENTE`
    pub user_id: &'r str,

    /// `COD_PRODUTO`
    pub item_id: u32,
    /// `NOME_PRODUTO`
    pub name: &'r str,
}

impl<'r> Record<'r> {
    /// Parses the `csv` into a `Vec<Self>`.
    pub fn parse(csv: &'r str) -> Vec<Self> {
        let mut lines = csv.lines();

        // skips the header of the csv first line
        lines.next();

        lines.filter_map(Self::parse_line).collect()
    }

    #[inline(always)]
    fn parse_line(line: &'r str) -> Option<Self> {
        let mut parts = line.split(',');

        let timestamp = parts.next()?.parse().ok()?;
        let user_id = parts.next()?;
        let item_id = parts.next()?.parse().ok()?;
        // for some reason, name has some spaces after the last char (?)
        // so we trim it :D
        let name = parts.next()?.trim();

        Some(Self {
            timestamp,
            user_id,
            item_id,
            name,
        })
    }
}

#[cfg(test)]
mod test {
    use super::Record;

    #[test]
    fn parse() {
        let csv = std::fs::read_to_string("dados_de_venda.csv").expect("Failed to load csv file");
        let original_len = csv.lines().count() - 1;

        let records = Record::parse(&csv);

        println!("{records:#?}");
        assert_eq!(records.len(), original_len);
    }
}
