use python310::Python;

fn main() {
    Python::default().pip().install_manifest();
}
