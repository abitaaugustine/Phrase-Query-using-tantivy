use tantivy::schema::{Schema, TEXT, STORED};
use tantivy::{doc, Index, Result};
use tantivy::query::QueryParser;

fn main() -> Result<()> {
    // Define the schema
    let mut schema_builder = Schema::builder();
    let content = schema_builder.add_text_field("content", TEXT | STORED);
    let schema = schema_builder.build();

    // Create the index
    let index = Index::create_in_ram(schema.clone());

    // Add documents to the index
    let mut index_writer = index.writer(50_000_000)?;
    index_writer.add_document(doc!(content => "the quick brown black fox jumps over the lazy dog"))?;
    index_writer.add_document(doc!(content => "the quick brown black blue fox jumps over the lazy dog"))?;
    index_writer.add_document(doc!(content => "the fast brown fox jumps over the sleepy dog"))?;
    index_writer.commit()?;

    // Create a reader and searcher
    let reader = index.reader()?;
    let searcher = reader.searcher();

    // Create the query parser
    let query_parser = QueryParser::for_index(&index, vec![content]);

    // Parse the phrase query
    let query = query_parser.parse_query("\"brown fox\"~1")?;

    // Execute the search
    let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(10))?;

    // Display the results
    for (score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!("Score: {:.2}", score);
        println!("{}", schema.to_json(&retrieved_doc));
    }

    Ok(())
}
