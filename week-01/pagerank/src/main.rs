/*
Ask 1: How does the PageRank algorithm calculate the importance of each node in the graph?
Answer: The PageRank algorithm, developed by Google's founders, is a way of measuring the importance
of website pages. It works by counting the number and quality of links to a page to determine a
rough estimate of how important the website is. The underlying assumption is that more important
websites are likely to receive more links from other websites.

Here's a simplified explanation of how it works:

    1. Start with a directed graph where each node represents a webpage and each edge represents a link from one page to another.

    2. Assign each node a preliminary rank value. This could be 1/N for a graph of N nodes.

    3. For each node, distribute its current rank value equally among its outgoing links.

    4. Each node's new rank becomes the sum of the rank values received from other nodes.

    5. Repeat steps 3 and 4 until the ranks converge (i.e., they don't change significantly between
    iterations).

    6. The final rank values are the PageRanks of the nodes.

Please note that this is a simplified explanation. The actual PageRank algorithm involves more
complex mathematical concepts and considerations.


Ask 2: What is the effect of the damping factor on the PageRank algorithm?
Answer: the algorithm includes a damping factor (usually set to 0.85) to account for the fact
that not all surfers will follow all links from a page.
This factor is applied in step 3, where each node distributes not its full rank, but a portion of it.


Ask 3: What is the purpose of running the algorithm for a certain number of iterations?
Answer: In many algorithms, especially those involving optimization or approximation, running the
algorithm for a certain number of iterations serves a few purposes:

    1. Convergence: Some algorithms are iterative in nature and reach their goal gradually.
    They start with an initial guess and improve upon it with each iteration.
    After a certain number of iterations, the solution may converge, meaning it doesn't
    significantly change after that point.

    2. Performance: Limiting the number of iterations can control the computational cost.
    More iterations usually mean more computation time and resources.
    By setting a limit, you can balance between getting a good enough solution and not using
    excessive resources.

    3. Preventing infinite loops: In some cases, an algorithm might keep running indefinitely
    without reaching a satisfactory solution. Setting a maximum number of iterations can prevent
    this.

In the context of the PageRank algorithm, it's typically run for many iterations until the ranks of
the pages (nodes) converge, i.e., they don't change significantly between iterations.
The number of iterations needed can depend on the size and structure of the web graph and the degree
of precision desired in the PageRank values.
*/

// Importing the fill function from the textwrap crate to wrap text at 78 characters per line.
use textwrap::fill;

// The PageRank struct holds the damping factor and the number of iterations to run the algorithm.
struct PageRank {
    damping: f64,
    iterations: usize,
}

impl PageRank {
    // The new function creates a new instance of the PageRank struct.
    fn new(damping: f64, iterations: usize) -> Self {
        Self {
            damping,
            iterations,
        }
    }

    // The rank function calculates and returns the PageRank for each node in the graph.
    fn rank(&self, graph: &Vec<Vec<usize>>) -> Vec<f64> {
        // The number of nodes in the graph.
        let n = graph.len();

        // The initial PageRank value for each node.
        let mut ranks = vec![1.0 / (n as f64); n];

        // Iterates the specified number of times.
        for _ in 0..self.iterations {
            // A new vector to hold the updated PageRank values.
            let mut new_ranks = vec![0.0; n];

            // Iterates over each node and its edges in the graph.
            for (node, edges) in graph.iter().enumerate() {
                // The amount of PageRank value this node contributes to its linked nodes.
                let contribution = ranks[node] / (edges.len() as f64);

                // Distributes the PageRank value to the linked nodes.
                for &edge in edges {
                    new_ranks[edge] += contribution;
                }
            }

            // Updates the PageRank values using the damping factor.
            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }

            // Replaces the old PageRank values with the new ones.
            ranks = new_ranks;
        }

        // Returns the final PageRank values.
        ranks
    }
}

fn main() {
    // The graph represents links between sports websites. Each index represents a website,
    // and the values in the vectors are the indexes of the websites they link to.
    let graph = vec![
        vec![1, 2], // ESPN links to NFL, NBA
        vec![0],    // NFL links to ESPN
        vec![0, 3], // NBA links to ESPN, UFC
        vec![0],    // UFC links to ESPN
        vec![0, 1], // MLB links to ESPN, NFL
    ];

    // The names corresponding to the indexes of the websites.
    let names = vec!["ESPN", "NFL", "NBA", "UFC", "MLB"];

    // Initializes the PageRank struct.
    let pagerank = PageRank::new(0.85, 100);

    // Calculates the PageRank values.
    let ranks = pagerank.rank(&graph);

    // Prints the PageRank values.
    for (i, rank) in ranks.iter().enumerate() {
        println!("The PageRank of {} is {}", names[i], rank);
    }

    // Explanation of how PageRank works.
    let explanation = "PageRank is a link analysis algorithm used by Google that uses the hyperlink structure of the web to determine a quality ranking for each web page. It works by counting the number and quality of links to a page to determine a rough estimate of how important the website is.";

    // Prints the explanation wrapped at 78 characters per line.
    println!("{}", fill(explanation, 78));
}
