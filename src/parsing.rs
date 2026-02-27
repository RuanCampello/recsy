//! This module takes care of `csv` parsing of the user and products file.

/// Represents each element inside the `csv`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Record<'r> {
    /// `DATA_COMPRA`: data em que a compra foi realizada, no formato aaaammdd;
    pub timestamp: u32,

    /// `COD_CLIENTE`: código único que identifica o cliente no sistema de informação da loja;
    pub user_id: &'r str,

    /// `COD_PRODUTO`: código único que identifica o produto no sistema de informação da loja;
    pub item_id: u32,
    /// `NOME_PRODUTO`: nome descritivo do produto adquirido.
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
        //
        // we also do trim_matches to remove outer csv quotes
        // now we get a clean string
        let name = parts.next()?.trim_matches('"').trim();

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
