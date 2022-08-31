pub struct Matrix4 {
    pub data: [f64; 16],
}

use std::ops::*;
impl Add for Matrix4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Matrix4 {
            data: [
                self.data[0] + rhs.data[0],
                self.data[1] + rhs.data[1],
                self.data[2] + rhs.data[2],
                self.data[3] + rhs.data[3],
                self.data[4] + rhs.data[4],
                self.data[5] + rhs.data[5],
                self.data[6] + rhs.data[6],
                self.data[7] + rhs.data[7],
                self.data[8] + rhs.data[8],
                self.data[9] + rhs.data[9],
                self.data[10] + rhs.data[10],
                self.data[11] + rhs.data[11],
                self.data[12] + rhs.data[12],
                self.data[13] + rhs.data[13],
                self.data[14] + rhs.data[14],
                self.data[15] + rhs.data[15],
            ],
        }
    }
}
impl Sub for Matrix4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix4 {
            data: [
                self.data[0] - rhs.data[0],
                self.data[1] - rhs.data[1],
                self.data[2] - rhs.data[2],
                self.data[3] - rhs.data[3],
                self.data[4] - rhs.data[4],
                self.data[5] - rhs.data[5],
                self.data[6] - rhs.data[6],
                self.data[7] - rhs.data[7],
                self.data[8] - rhs.data[8],
                self.data[9] - rhs.data[9],
                self.data[10] - rhs.data[10],
                self.data[11] - rhs.data[11],
                self.data[12] - rhs.data[12],
                self.data[13] - rhs.data[13],
                self.data[14] - rhs.data[14],
                self.data[15] - rhs.data[15],
            ],
        }
    }
}
impl Mul for Matrix4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Matrix4 {
            data: [
                self.data[0] * rhs.data[0]
                    + self.data[1] * rhs.data[4]
                    + self.data[2] * rhs.data[8]
                    + self.data[3] * rhs.data[12],
                self.data[0] * rhs.data[1]
                    + self.data[1] * rhs.data[5]
                    + self.data[2] * rhs.data[9]
                    + self.data[3] * rhs.data[13],
                self.data[0] * rhs.data[2]
                    + self.data[1] * rhs.data[6]
                    + self.data[2] * rhs.data[10]
                    + self.data[3] * rhs.data[14],
                self.data[0] * rhs.data[3]
                    + self.data[1] * rhs.data[7]
                    + self.data[2] * rhs.data[11]
                    + self.data[3] * rhs.data[15],
                self.data[4] * rhs.data[0]
                    + self.data[5] * rhs.data[4]
                    + self.data[6] * rhs.data[8]
                    + self.data[7] * rhs.data[12],
                self.data[4] * rhs.data[1]
                    + self.data[5] * rhs.data[5]
                    + self.data[6] * rhs.data[9]
                    + self.data[7] * rhs.data[13],
                self.data[4] * rhs.data[2]
                    + self.data[5] * rhs.data[6]
                    + self.data[6] * rhs.data[10]
                    + self.data[7] * rhs.data[14],
                self.data[4] * rhs.data[3]
                    + self.data[5] * rhs.data[7]
                    + self.data[6] * rhs.data[11]
                    + self.data[7] * rhs.data[15],
                self.data[8] * rhs.data[0]
                    + self.data[9] * rhs.data[4]
                    + self.data[10] * rhs.data[8]
                    + self.data[11] * rhs.data[12],
                self.data[8] * rhs.data[1]
                    + self.data[9] * rhs.data[5]
                    + self.data[10] * rhs.data[9]
                    + self.data[11] * rhs.data[13],
                self.data[8] * rhs.data[2]
                    + self.data[9] * rhs.data[6]
                    + self.data[10] * rhs.data[10]
                    + self.data[11] * rhs.data[14],
                self.data[8] * rhs.data[3]
                    + self.data[9] * rhs.data[7]
                    + self.data[10] * rhs.data[11]
                    + self.data[11] * rhs.data[15],
                self.data[12] * rhs.data[0]
                    + self.data[13] * rhs.data[4]
                    + self.data[14] * rhs.data[8]
                    + self.data[15] * rhs.data[12],
                self.data[12] * rhs.data[1]
                    + self.data[13] * rhs.data[5]
                    + self.data[14] * rhs.data[9]
                    + self.data[15] * rhs.data[13],
                self.data[12] * rhs.data[2]
                    + self.data[13] * rhs.data[6]
                    + self.data[14] * rhs.data[10]
                    + self.data[15] * rhs.data[14],
                self.data[12] * rhs.data[3]
                    + self.data[13] * rhs.data[7]
                    + self.data[14] * rhs.data[11]
                    + self.data[15] * rhs.data[15],
            ],
        }
    }
}
