//#[derive(Debug, Copy, Clone, PartialEq, Eq)] //PartialOrd, Ord
enum Node {
    WorkingSpace(&str) // will be something like "", ".", ".." 
}