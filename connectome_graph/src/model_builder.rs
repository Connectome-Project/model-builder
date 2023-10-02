use regex::Regex;

use crate::{graph_db_connection::GraphDbConnection, pattern::Pattern, GremlinConnectionType};
use std::{iter::Peekable, vec::IntoIter};

pub async fn build_model<Pat: Pattern, D: From<Pat>>(
    client: GremlinConnectionType,
    mut data_to_build: Peekable<IntoIter<Pat>>,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut pattern_so_far = Pat::default();
    if let Some(pattern_segment) = data_to_build.peek() {
        let extended_pattern = pattern_so_far.concat(pattern_segment.clone());

        // let res = client.get_nodes_by_label_regexp(Regex::new()).await?;
        //g.V().filter {it.get().label().length() <= "hellooo".length()}
        /*
        g.V()
        .map{ vertex -> 
            label = vertex.label()   // Get the label of the vertex
            regexPattern = "\\b${label}\\b" // Create a regex pattern using the label
            [vertex, regexPattern]
        }
        .filter{ _, regexPattern ->
            // Check if the input matches the regex pattern
            input.matches(regexPattern)
        }
        .select{ vertex, _ ->
            // Output the vertices that match the input
            vertex
        }


g.V()  \
.map{ vertex -> \
    def label = vertex.label()  \
    regexPattern = "\\b${label}\\b" \
    [vertex, regexPattern] \
} \
.filter{ _, regexPattern -> \
    input.matches(regexPattern) \
} \
.select{ vertex, _ -> \
    vertex \
}

        */
    }
    Ok(())
}
