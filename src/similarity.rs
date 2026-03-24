//! This module implements the `Similarity` module.
//!
//! It receives data from [`ShoppingList`], builds a dense purchase matrix,
//! computes the intersection matrix via standard matrix multiplication,
//! and derives the asymmetric Jaccard similarity matrix.

use crate::shopping::ShoppingList;

pub struct Similarity {
    /// similarity matrix of dimension `n × n`.
    /// `similarity[i * n + j]` = Jaccard distance from client `i` to client `j`.
    similarity: Vec<Vec<f64>>,
}

impl Similarity {
    /// Builds the similarity matrix from a [`ShoppingList`].
    pub fn new(list: &ShoppingList) -> Self {
        let n = list.client_count();
        let m = list.product_count();

        // 1: dense purchase matrix
        let mut a = vec![vec![0.0; m]; n];
        for (client, products) in list.purchases().iter().enumerate() {
            for &product in products {
                a[client][product] = 1.0;
            }
        }

        // 2: transpose the matrix
        let a_transposed = Self::transpose(&a);
        // 3: intersection matrix
        let intersection = Self::multiply(&a, &a_transposed);

        // 4: build similarity matrix
        // we could start with 1.0 but whatever
        let mut similarity = vec![vec![0.0; n]; n];

        for i in 0..n {
            let pi = intersection[i][i]; // [i][i] is the total of products i

            for j in 0..n {
                match pi > 0.0 {
                    true => similarity[i][j] = 1.0 - (intersection[i][j] / pi),
                    false => similarity[i][j] = 1.0,
                }
            }
        }

        Self { similarity }
    }

    fn transpose(a: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let m = a.len();
        let n = a[0].len();

        let mut transposed = vec![vec![0.0; m]; n];
        for i in 0..m {
            for j in 0..n {
                transposed[j][i] = a[i][j];
            }
        }

        transposed
    }

    fn multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let m = a.len();
        let n = a[0].len();
        let p = b[0].len();
        let mut c = vec![vec![0.0; p]; m];

        for i in 0..m {
            for j in 0..p {
                let mut sum = 0.0;
                for k in 0..n {
                    sum += a[i][k] * b[k][j];
                }

                c[i][j] = sum
            }
        }

        c
    }

    #[inline(always)]
    pub fn similarity(&self, from: usize, to: usize) -> f64 {
        self.similarity[from][to]
    }

    pub fn most_similar(&self, client: usize) -> usize {
        let mut min = f64::MAX;
        let mut most_similary = client;

        for (j, &value) in self.similarity[client].iter().enumerate() {
            if j == client {
                continue; // we ignore the client it self, of course :D
            }

            if value < min {
                min = value;
                most_similary = j;
            }
        }

        most_similary
    }
}

#[cfg(test)]
mod test {
    use super::Similarity;
    use crate::parsing::Record;
    use crate::shopping::ShoppingList;

    #[test]
    fn find_most_similar_for_two_clients() {
        let csv = std::fs::read_to_string("dados_de_venda.csv").unwrap();
        let records = Record::parse(&csv);
        let list = ShoppingList::new(&records);
        let sim = Similarity::new(&list);

        let clients = [0, 1];
        for &client in &clients {
            let similar = sim.most_similar(client);
            assert_ne!(
                similar, client,
                "most_similar must not return the client itself"
            );
            println!("client {client} -> most similar: {similar}");
        }
    }
}
