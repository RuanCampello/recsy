//! Recommendation module.
//!
//! Implements the ranking described in the project definition.

use crate::{shopping::ShoppingList, similarity::Similarity};

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

        // we convert the sparse matrix of purchases
        // into a dense vector of bools to do lookups
        let mut bought = vec![false; products];
        for &product in self.list.purchases()[client].iter() {
            bought[product] = true;
        }

        let mut rank = vec![1.0; products];

        // go through all neighbours s
        for s in 0..clients {
            if s == client {
                continue; // ignore itself
            }

            let similarity = self.matrix.similarity(client, s);
            // we ignore less then 1 similarities cause it means that
            // our intersection with them is empty
            if similarity >= 1.0 {
                continue;
            }

            for &product in &self.list.purchases()[s] {
                rank[product] *= similarity;
            }
        }

        // now we know which products has the client bought,
        // we can exclude them from being potential candidates to be purchased :D
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

        candidates.truncate(k);
        candidates
    }

    #[inline(always)]
    pub fn recommend_for_client_code(&self, code: &str, k: usize) -> Vec<RankedProduct> {
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
                let name = list
                    .product_name(item.product_id)
                    .expect("product name to exists");
                println!("  - {} (score={:4e})", name, item.rank);
            }
        }
    }
}
