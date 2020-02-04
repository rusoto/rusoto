use skeptic::markdown_files_of_directory;

fn main() {
  let mdbook_files = markdown_files_of_directory("../");
  skeptic::generate_doc_tests(&mdbook_files);
}
