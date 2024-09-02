
pub mod rename_image {
    pub fn single() -> std::io::Result<()> {
        std::fs::rename("a.txt", "b.txt")?; // Rename a.txt to b.txt
        Ok(())
    }
    pub fn directory() -> std::io::Result<()> {
        println!("You are using directories!");
        Ok(())
    }
}
