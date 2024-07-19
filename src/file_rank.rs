
pub const RANK_8: u64 =  0xFF << (8 * 0);
pub const RANK_7: u64 =  0xFF << (8 * 1);
pub const RANK_6: u64 =  0xFF << (8 * 2);
pub const RANK_5: u64 =  0xFF << (8 * 3);
pub const RANK_4: u64 =  0xFF << (8 * 4);
pub const RANK_3: u64 =  0xFF << (8 * 5);
pub const RANK_2: u64 =  0xFF << (8 * 6);
pub const RANK_1: u64 =  0xFF << (8 * 7);

pub const RANK_1_2:u64 = RANK_1 | RANK_2;
pub const RANK_7_8:u64 = RANK_7 | RANK_8;

pub const NOT_RANK_1:u64 = !RANK_1;
pub const NOT_RANK_1_2:u64 = !RANK_1_2;

pub const NOT_RANK_8:u64 = !RANK_8;
pub const NOT_RANK_7_8:u64 = !RANK_7_8;

/*   
Example of not A file
   a b c d e f g h
8| 0 1 1 1 1 1 1 0 
7| 0 1 1 1 1 1 1 0
6| 0 1 1 1 1 1 1 0
5| 0 1 1 1 1 1 1 0
4| 0 1 1 1 1 1 1 0
3| 0 1 1 1 1 1 1 0
2| 0 1 1 1 1 1 1 0
1| 0 1 1 1 1 1 1 0*/

pub const FILE_NOT_A:u64 = 18374403900871474942;
pub const FILE_NOT_B:u64 = 18302063728033398269;
pub const FILE_NOT_AB:u64 = 18229723555195321596;
pub const FILE_NOT_H:u64 = 9187201950435737471;
pub const FILE_NOT_G:u64 = 13816973012072644543;
pub const FILE_NOT_GH:u64 = 4557430888798830399;

pub const FILE_A:u64 = !FILE_NOT_A;
pub const FILE_H:u64 = !FILE_NOT_H;

pub const EDGES:u64 = RANK_1 | RANK_8 | FILE_H | FILE_A;