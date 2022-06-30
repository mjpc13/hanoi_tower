use std::iter;
use std::error::Error;
use std::fmt;

pub struct Disk{
    pub number: i32,
    pub string: String,
}

impl Disk{
    pub fn new(number: i32, num_disks: &i32) -> Disk {
        let mut string = String::new();

        for _ in 0..(num_disks-(number-1)/2){string.push(' ');}

        for _ in 0..number{string.push('+');}

        for _ in 0..(num_disks-(number-1)/2){string.push(' ');}

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
            for n in 0..*n_disks{
                disks.push(Disk::new(2*n+1, n_disks));
            }
        }

        Stack{disks, fill}
    }

    pub fn count_disks(&self) -> i32 { self.disks.len().try_into().unwrap() }

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
    n_disks: i32,
    string: String,
}

impl Board{
    
    pub fn new(n_disks: &i32) -> Board {
        
        let mut s1 = Stack::new(n_disks, true);
        let mut s2 = Stack::new(n_disks, true);
        let mut s3 = Stack::new(n_disks, false);
        let mut stacks = [s1,s2,s3];

        let height: i32 = n_disks+3;
        let width: i32 = ((n_disks*2+1))*3;

        let mut string: String = String::new();

        Board{stacks,n_disks: *n_disks, height, width, string} 
    }

    fn draw_empty(&self) -> String {
        
        let mut tmp = String::new();
        for _ in 0..self.n_disks { tmp.push(' '); }
        tmp.push('|');
        for _ in 0..self.n_disks { tmp.push(' '); }

        tmp
    }

    pub fn update_string(&mut self){

        //Draw the first line (always empty)
        let mut tmp = String::new();

        for _ in 0..3 {
            tmp.push_str(
                &self.draw_empty()
            );
        }
        tmp.push('\n');

        //TODO:  draw the stacks
        for i in 0..(self.n_disks){
            for j in 0..3{
                if self.stacks[j].count_disks() < (i-self.n_disks).abs(){
                    tmp.push_str(&self.draw_empty());
                }
                else{
                    tmp.push_str(&self.stacks[j].disks[i as usize].string);
                }
            }
            tmp.push('\n');
        }

        for _ in 0..self.width { tmp.push('#'); }
        self.string = tmp;
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}
