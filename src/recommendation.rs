//! Recommendation module.
//!
//! Implements the ranking described in the project definition.

use crate::{
    shopping::ShoppingList,
    similarity::{self, Similarity},
};

pub struct Recommendation<'l> {
    matrix: Similarity,
    list: &'l ShoppingList<'l>,
}

pub struct RankedProduct {
    product_id: usize,
    rank: f64,
}

impl<'l> Recommendation<'l> {
    pub fn new(matrix: Similarity, list: &'l ShoppingList<'l>) -> Self {
        Self { matrix, list }
    }

    /// Recommends `k` products for the given client identified by the original code.
    pub fn recommend_for_client(&self, client: usize, k: usize) -> Vec<RankedProduct> {
        let clients = self.list.client_count();
        let products = self.list.product_count();

        let mut bought = vec![false; products];
        for &product in self.list.purchases()[client].iter() {
            if product < products {
                bought[product] = true;
            }
        }

        let mut rank = vec![1.0; products];

        // go through all neighbours s
        for s in 0..clients {
            if s == client {
                continue; // ignore itself
            }

            let similarity = self.matrix.similarity(client, s);
            if similarity >= 1.0 {
                continue;
            }

            for &product in &self.list.purchases()[s] {
                if !bought[product] {
                    rank[product] += similarity;
                }
            }
        }

        let mut candidates = Vec::new();
        for product in 0..products {
            if !bought[product] {
                candidates.push(RankedProduct {
                    product_id: product,
                    rank: rank[product],
                })
            }
        }

        candidates.sort_by(|a, b| {
            a.rank
                .partial_cmp(&b.rank)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        candidates.truncate(k.min(candidates.len()));
        candidates
    }

    #[inline(always)]
    fn recommend_for_client_code(&self, code: &str, k: usize) -> Vec<RankedProduct> {
        let client = self.list.client_index(code).expect("client to exists");
        self.recommend_for_client(client, k)
    }
}

#[cfg(test)]
mod test {
    use super::Recommendation;
    use crate::parsing::Record;
    use crate::shopping::ShoppingList;
    use crate::similarity::Similarity;

    #[test]
    fn recommend_k_products() {
        let csv = std::fs::read_to_string("dados_de_venda.csv").unwrap();
        let records = Record::parse(&csv);
        let list = ShoppingList::new(&records);
        let sim = Similarity::new(&list);
        let rec = Recommendation::new(sim, &list);

        let clients = ["99BMYG01", "9O09ND01", "93311201"];
        let k = 5;

        for code in clients {
            let ranked = rec.recommend_for_client_code(code, k);

            println!("Recommendations for client {code}:");
            for item in ranked {
                let name = list.product_name(item.product_id).unwrap_or("<unknown>");
                println!("  - {} (score={})", name, item.rank);
            }
        }
    }
}

