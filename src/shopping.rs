//! This module implements the `ShoppingList` module.
//!
//! It takes the parsed CSV records and organises them into indexed
//! structure that similarity and recommendation will consume.

use crate::map::HashMap;
use crate::parsing::Record;

/// Holds the purchase data indexed for fast lookup.
pub struct ShoppingList<'l> {
    /// The position inside this vector **is** the internal client index.
    clients: Vec<&'l str>,
    /// Reverse lookup from client index to internal index.
    client_map: HashMap<&'l str, usize>,

    /// The position inside this vector **is** the internal product index.
    products: Vec<&'l str>,
    /// Reverse lookup from product code to internal index.
    product_map: HashMap<u32, usize>,

    /// List of internal product ids that the client has purchased.
    purchases: Vec<Vec<usize>>,
}

impl<'l> ShoppingList<'l> {
    pub fn new(records: &[Record<'l>]) -> Self {
        let mut clients = Vec::new();
        let mut client_map = HashMap::new();

        let mut products = Vec::new();
        let mut product_map = HashMap::new();

        // first pass: we get every unique client and product
        // to fill the vectors and maps
        for record in records {
            // register the client if we haven't seen it yet
            if client_map.get(record.user_id).is_none() {
                let idx = clients.len();
                clients.push(record.user_id);
                client_map.insert(record.user_id, idx);
            }

            // register the product if we haven't seen it yet
            if product_map.get(&record.item_id).is_none() {
                let idx = products.len();
                products.push(record.name);
                product_map.insert(record.item_id, idx);
            }
        }

        let mut purchases = (0..clients.len()).map(|_| Vec::new()).collect::<Vec<_>>();

        // second pass: we populate each client purchase with the internal index
        for record in records {
            let client_idx = client_map[record.user_id];
            let product_idx = product_map[&record.item_id];
            purchases[client_idx].push(product_idx);
        }

        Self {
            clients,
            client_map,
            products,
            product_map,
            purchases,
        }
    }

    #[inline(always)]
    pub fn client_products(&self, client: &str) -> Option<&[usize]> {
        let &idx = self.client_map.get(client)?;
        Some(&self.purchases[idx])
    }

    #[inline(always)]
    pub fn product_name(&self, index: usize) -> Option<&str> {
        self.products.get(index).copied()
    }

    #[inline(always)]
    pub fn client_count(&self) -> usize {
        self.clients.len()
    }

    #[inline(always)]
    pub fn product_count(&self) -> usize {
        self.products.len()
    }

    #[inline(always)]
    pub fn purchases(&self) -> &[Vec<usize>] {
        &self.purchases
    }
}

#[cfg(test)]
mod test {
    use super::ShoppingList;
    use crate::parsing::Record;

    #[test]
    fn unknown_client_returns_none() {
        let csv = std::fs::read_to_string("dados_de_venda.csv").unwrap();
        let records = Record::parse(&csv);
        let list = ShoppingList::new(&records);

        assert!(list.client_products("INVALID_CODE").is_none());
    }

    #[test]
    fn show_products_for_three_clients() {
        let csv = std::fs::read_to_string("dados_de_venda.csv").unwrap();
        let records = Record::parse(&csv);
        let list = ShoppingList::new(&records);

        let clients = ["99BMYG01", "9O09ND01", "93311201"];

        for code in clients {
            let products = list
                .client_products(code)
                .expect("sample client should exist");
            assert!(!products.is_empty());

            println!("client {code} bought:");
            for &pid in products {
                let name = list.product_name(pid).unwrap();
                println!("  - {name}");
            }
        }
    }
}
