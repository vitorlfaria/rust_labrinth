use crate::utils::frame::{Frame, new_frame};

pub fn generate_lose_screen() -> Frame {
    let mut lose_screen = new_frame();

    lose_screen[23][6] = "▓".to_string();
    lose_screen[24][6] = "█".to_string();
    lose_screen[25][6] = "█".to_string();
    lose_screen[29][6] = "█".to_string();
    lose_screen[30][6] = "█".to_string();
    lose_screen[31][6] = "▓".to_string();
    lose_screen[34][6] = "█".to_string();
    lose_screen[35][6] = "█".to_string();
    lose_screen[36][6] = "█".to_string();
    lose_screen[37][6] = "█".to_string();
    lose_screen[38][6] = "█".to_string();
    lose_screen[42][6] = "█".to_string();
    lose_screen[47][6] = "█".to_string();
    lose_screen[48][6] = "█".to_string();

    lose_screen[25][7] = "█".to_string();
    lose_screen[26][7] = "█".to_string();
    lose_screen[29][7] = "█".to_string();
    lose_screen[30][7] = "█".to_string();
    lose_screen[33][7] = "█".to_string();
    lose_screen[34][7] = "█".to_string();
    lose_screen[38][7] = "█".to_string();
    lose_screen[39][7] = "█".to_string();
    lose_screen[42][7] = "█".to_string();
    lose_screen[43][7] = "█".to_string();
    lose_screen[46][7] = "▓".to_string();
    lose_screen[47][7] = "█".to_string();
    lose_screen[48][7] = "█".to_string();

    lose_screen[26][8] = "█".to_string();
    lose_screen[27][8] = "█".to_string();
    lose_screen[29][8] = "█".to_string();
    lose_screen[30][8] = "█".to_string();
    lose_screen[33][8] = "█".to_string();
    lose_screen[34][8] = "█".to_string();
    lose_screen[38][8] = "█".to_string();
    lose_screen[39][8] = "█".to_string();
    lose_screen[41][8] = "▓".to_string();
    lose_screen[42][8] = "█".to_string();
    lose_screen[43][8] = "█".to_string();
    lose_screen[47][8] = "█".to_string();
    lose_screen[48][8] = "█".to_string();

    lose_screen[27][9] = "▐".to_string();
    lose_screen[28][9] = "█".to_string();
    lose_screen[29][9] = "█".to_string();
    lose_screen[30][9] = "▓".to_string();
    lose_screen[33][9] = "█".to_string();
    lose_screen[34][9] = "█".to_string();
    lose_screen[38][9] = "█".to_string();
    lose_screen[39][9] = "█".to_string();
    lose_screen[41][9] = "▓".to_string();
    lose_screen[42][9] = "▓".to_string();
    lose_screen[43][9] = "█".to_string();
    lose_screen[47][9] = "█".to_string();
    lose_screen[48][9] = "█".to_string();

    lose_screen[27][10] = "█".to_string();
    lose_screen[28][10] = "█".to_string();
    lose_screen[30][10] = "▓".to_string();
    lose_screen[34][10] = "█".to_string();
    lose_screen[35][10] = "█".to_string();
    lose_screen[36][10] = "█".to_string();
    lose_screen[37][10] = "█".to_string();
    lose_screen[38][10] = "▓".to_string();
    lose_screen[43][10] = "█".to_string();
    lose_screen[44][10] = "█".to_string();
    lose_screen[45][10] = "█".to_string();
    lose_screen[46][10] = "█".to_string();
    lose_screen[47][10] = "█".to_string();
    lose_screen[48][10] = "▓".to_string();

    lose_screen[26][11] = "█".to_string();
    lose_screen[27][11] = "█".to_string();
    lose_screen[43][11] = "▓".to_string();

    lose_screen[24][12] = "▓".to_string();
    lose_screen[25][12] = "█".to_string();
    lose_screen[26][12] = "█".to_string();

    lose_screen[22][16] = "█".to_string();
    lose_screen[23][16] = "█".to_string();
    lose_screen[24][16] = "▓".to_string();
    lose_screen[31][16] = "█".to_string();
    lose_screen[32][16] = "█".to_string();
    lose_screen[33][16] = "█".to_string();
    lose_screen[34][16] = "█".to_string();
    lose_screen[35][16] = "█".to_string();
    lose_screen[40][16] = "█".to_string();
    lose_screen[41][16] = "█".to_string();
    lose_screen[42][16] = "█".to_string();
    lose_screen[43][16] = "█".to_string();
    lose_screen[44][16] = "█".to_string();
    lose_screen[45][16] = "█".to_string();
    lose_screen[47][16] = "▓".to_string();
    lose_screen[48][16] = "█".to_string();
    lose_screen[49][16] = "█".to_string();
    lose_screen[50][16] = "█".to_string();
    lose_screen[51][16] = "█".to_string();
    lose_screen[52][16] = "█".to_string();

    lose_screen[21][17] = "▓".to_string();
    lose_screen[22][17] = "█".to_string();
    lose_screen[23][17] = "█".to_string();
    lose_screen[30][17] = "█".to_string();
    lose_screen[31][17] = "█".to_string();
    lose_screen[35][17] = "█".to_string();
    lose_screen[36][17] = "█".to_string();
    lose_screen[39][17] = "█".to_string();
    lose_screen[40][17] = "█".to_string();
    lose_screen[47][17] = "▓".to_string();
    lose_screen[48][17] = "█".to_string();
    lose_screen[52][17] = "▀".to_string();
    
    lose_screen[22][18] = "█".to_string();
    lose_screen[23][18] = "█".to_string();
    lose_screen[30][18] = "█".to_string();
    lose_screen[31][18] = "█".to_string();
    lose_screen[35][18] = "█".to_string();
    lose_screen[36][18] = "█".to_string();
    lose_screen[40][18] = "▓".to_string();
    lose_screen[41][18] = "█".to_string();
    lose_screen[42][18] = "█".to_string();
    lose_screen[43][18] = "▄".to_string();
    lose_screen[48][18] = "█".to_string();
    lose_screen[49][18] = "█".to_string();
    lose_screen[50][18] = "█".to_string();

    lose_screen[22][19] = "█".to_string();
    lose_screen[23][19] = "█".to_string();
    lose_screen[30][19] = "█".to_string();
    lose_screen[31][19] = "█".to_string();
    lose_screen[35][19] = "█".to_string();
    lose_screen[36][19] = "█".to_string();
    lose_screen[44][19] = "█".to_string();
    lose_screen[45][19] = "█".to_string();
    lose_screen[48][19] = "▓".to_string();
    lose_screen[49][19] = "█".to_string();
    lose_screen[52][19] = "▄".to_string();

    lose_screen[22][20] = "█".to_string();
    lose_screen[23][20] = "█".to_string();
    lose_screen[24][20] = "█".to_string();
    lose_screen[25][20] = "█".to_string();
    lose_screen[26][20] = "█".to_string();
    lose_screen[27][20] = "█".to_string();
    lose_screen[31][20] = "█".to_string();
    lose_screen[32][20] = "█".to_string();
    lose_screen[33][20] = "█".to_string();
    lose_screen[34][20] = "█".to_string();
    lose_screen[35][20] = "▓".to_string();
    lose_screen[39][20] = "█".to_string();
    lose_screen[40][20] = "█".to_string();
    lose_screen[41][20] = "█".to_string();
    lose_screen[42][20] = "█".to_string();
    lose_screen[43][20] = "█".to_string();
    lose_screen[44][20] = "█".to_string();
    lose_screen[49][20] = "█".to_string();
    lose_screen[50][20] = "█".to_string();
    lose_screen[51][20] = "█".to_string();
    lose_screen[52][20] = "█".to_string();

    lose_screen[26][21] = "▓".to_string();
    lose_screen[42][21] = "▓".to_string();

    lose_screen
}
