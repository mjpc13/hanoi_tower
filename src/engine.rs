use std::iter;
use std::error::Error;
use std::fmt;

pub struct Disk{
    pub number: i32,
    pub string: String,
}
impl Disk{
    pub fn new(number: i32) -> Disk {
        let mut string = String::new();
        for n in 0..number{
            string.push('-');
        }

        Disk{number, string}
    }
}

pub struct Stack{
    pub disks: Vec<Disk>,
    pub fill: bool,
}

impl Stack {
    pub fn new(n_disks: &i32, fill: bool) -> Stack {

        let mut disks: Vec<Disk> = Vec::new();

        if fill{
            for number in 1..n_disks+1{
                disks.push(Disk::new(number));
            }
        }

        Stack{disks, fill}
    }

    pub fn move_disk(&self, other_stack: Stack) -> Result<(), &'static str>{
       if self.disks.last().unwrap().number <= other_stack.disks.last().unwrap().number{
           return Err("The move is illegal");
       }
       else{
           return Ok(());
       }

    }
}

pub struct Board{
    pub stacks: [Stack; 3],
    height: i32,
    width: i32,
}

impl Board{
    
    pub fn new(n_disks: &i32) -> Board {
        
        let mut s1 = Stack::new(n_disks, true);
        let mut s2 = Stack::new(n_disks, true);
        let mut s3 = Stack::new(n_disks, false);
        let mut stacks = [s1,s2,s3];

        let height: i32 = n_disks+3;
        let width: i32 = ((n_disks*2+1)+2)*3;

        Board{stacks,height, width} 
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string: String = String::new();
        write!(f, "{}", string)
    }
}
