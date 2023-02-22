
#[derive(Clone, Copy)]
enum Cell{
    None,
    Cross,
    Fill,
}

impl std::fmt::Display for Cell{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a  = match self{
            Cell::None  => " ",
            Cell::Cross => "x",
            Cell::Fill  => "#",
        };
        f.write_str(a)
    }
}

type Line<const SIZE: usize> = [Cell; SIZE];
struct Wrapper<const SIZE: usize>(Line<SIZE>);

impl<const SIZE: usize> std::fmt::Display for Wrapper<SIZE>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        string.push('|');
        for cell in self.0{
            string.push_str(format!("{}", cell).as_str());
        }
        string.push('|');
        f.write_str(string.as_str())
    }
}

fn main() {
    let nums = [8];
    let row = [Cell::None; 15];
}
