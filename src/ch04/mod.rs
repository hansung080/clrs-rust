mod matrix_multiply;
mod matrix_multiply_recursive;
mod matrix_multiply_strassen;

pub use matrix_multiply::*;
pub use matrix_multiply_recursive::*;
pub use matrix_multiply_strassen::*;

#[cfg(test)]
mod tests {
    use crate::utils::matrix::Mat;

    pub fn matrix_multiply_i32_n0(mat_mul: fn(&Mat<i32, 0, 0>, &Mat<i32, 0, 0>) -> Mat<i32, 0, 0>) {
        assert_eq!(mat_mul(&Mat([]), &Mat([])), Mat::<i32, 0, 0>([]));
    }

    pub fn matrix_multiply_i32_n1(mat_mul: fn(&Mat<i32, 1, 1>, &Mat<i32, 1, 1>) -> Mat<i32, 1, 1>) {
        assert_eq!(mat_mul(&Mat([[2]]), &Mat([[3]])), Mat([[6]]));
    }

    pub fn matrix_multiply_i32_n2(mat_mul: fn(&Mat<i32, 2, 2>, &Mat<i32, 2, 2>) -> Mat<i32, 2, 2>) {
        assert_eq!(mat_mul(&Mat(
            [
                [1, 2],
                [3, 4],
            ]
        ), &Mat(
            [
                [5, 6],
                [7, 8],
            ]
        )), Mat(
            [
                [19, 22],
                [43, 50],
            ]
        ));
    }

    pub fn matrix_multiply_i32_n3(mat_mul: fn(&Mat<i32, 3, 3>, &Mat<i32, 3, 3>) -> Mat<i32, 3, 3>) {
        assert_eq!(mat_mul(&Mat(
            [
                [1, 2, 3],
                [4, 5, 6],
                [7, 8, 9],
            ]
        ), &Mat(
            [
                [10, 11, 12],
                [13, 14, 15],
                [16, 17, 18],
            ]
        )), Mat(
            [
                [ 84,  90,  96],
                [201, 216, 231],
                [318, 342, 366],
            ]
        ));
    }

    pub fn matrix_multiply_i32_n4(mat_mul: fn(&Mat<i32, 4, 4>, &Mat<i32, 4, 4>) -> Mat<i32, 4, 4>) {
        assert_eq!(mat_mul(&Mat(
            [
                [ 1,  2,  3,  4],
                [ 5,  6,  7,  8],
                [ 9, 10, 11, 12],
                [13, 14, 15, 16],
            ]
        ), &Mat(
            [
                [17, 18, 19, 20],
                [21, 22, 23, 24],
                [25, 26, 27, 28],
                [29, 30, 31, 32],
            ]
        )), Mat(
            [
                [ 250,  260,  270,  280],
                [ 618,  644,  670,  696],
                [ 986, 1028, 1070, 1112],
                [1354, 1412, 1470, 1528],
            ]
        ));
    }

    pub fn matrix_multiply_i32_n8(mat_mul: fn(&Mat<i32, 8, 8>, &Mat<i32, 8, 8>) -> Mat<i32, 8, 8>) {
        assert_eq!(mat_mul(&Mat(
            [
                [ 1,  2,  3,  4,  5,  6,  7,  8],
                [ 9, 10, 11, 12, 13, 14, 15, 16],
                [17, 18, 19, 20, 21, 22, 23, 24],
                [25, 26, 27, 28, 29, 30, 31, 32],
                [33, 34, 35, 36, 37, 38, 39, 40],
                [41, 42, 43, 44, 45, 46, 47, 48],
                [49, 50, 51, 52, 53, 54, 55, 56],
                [57, 58, 59, 60, 61, 62, 63, 64],
            ]
        ), &Mat(
            [
                [ 65,  66,  67,  68,  69,  70,  71,  72],
                [ 73,  74,  75,  76,  77,  78,  79,  80],
                [ 81,  82,  83,  84,  85,  86,  87,  88],
                [ 89,  90,  91,  92,  93,  94,  95,  96],
                [ 97,  98,  99, 100, 101, 102, 103, 104],
                [105, 106, 107, 108, 109, 110, 111, 112],
                [113, 114, 115, 116, 117, 118, 119, 120],
                [121, 122, 123, 124, 125, 126, 127, 128],
            ]
        )), Mat(
            [
                [ 3684,  3720,  3756,  3792,  3828,  3864,  3900,  3936],
                [ 9636,  9736,  9836,  9936, 10036, 10136, 10236, 10336],
                [15588, 15752, 15916, 16080, 16244, 16408, 16572, 16736],
                [21540, 21768, 21996, 22224, 22452, 22680, 22908, 23136],
                [27492, 27784, 28076, 28368, 28660, 28952, 29244, 29536],
                [33444, 33800, 34156, 34512, 34868, 35224, 35580, 35936],
                [39396, 39816, 40236, 40656, 41076, 41496, 41916, 42336],
                [45348, 45832, 46316, 46800, 47284, 47768, 48252, 48736],
            ]
        ));
    }

    pub fn matrix_multiply_f64_n4(mat_mul: fn(&Mat<f64, 4, 4>, &Mat<f64, 4, 4>) -> Mat<f64, 4, 4>) {
        assert_eq!(mat_mul(&Mat(
            [
                [ 1.0,  2.0,  3.0,  4.0],
                [ 5.0,  6.0,  7.0,  8.0],
                [ 9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        ), &Mat(
            [
                [17.0, 18.0, 19.0, 20.0],
                [21.0, 22.0, 23.0, 24.0],
                [25.0, 26.0, 27.0, 28.0],
                [29.0, 30.0, 31.0, 32.0],
            ]
        )), Mat(
            [
                [ 250.0,  260.0,  270.0,  280.0],
                [ 618.0,  644.0,  670.0,  696.0],
                [ 986.0, 1028.0, 1070.0, 1112.0],
                [1354.0, 1412.0, 1470.0, 1528.0],
            ]
        ));
    }
}