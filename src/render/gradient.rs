use bevy::ecs::system::Resource;
use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::math::transformations::map_range;

#[derive(Resource, Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum Gradient {
    #[default]
    Turbo,
    Viridis,
    Magma,
    Plasma,
    Inferno,
    Bw,
}

impl Gradient {
    /// Returns the color at a certain percentage (between 0.0..=1.0) of the gradient
    pub fn at(&self, percent: f32, min: f32, max: f32) -> Color32 {
        let idx = map_range(min, max, 0., 255., percent) as u8 as usize;
        let map = match self {
            Gradient::Turbo => &TURBO,
            Gradient::Magma => &MAGMA,
            Gradient::Viridis => &VIRIDIS,
            Gradient::Plasma => &PLASMA,
            Gradient::Inferno => &INFERNO,
            Gradient::Bw => &BW,
        };

        let [r, g, b] = map[idx];
        Color32::from_rgb(r, g, b)
    }
}

// https://gist.github.com/mikhailov-work/6a308c20e494d9e0ccc29036b28faa7a
const TURBO: [[u8; 3]; 256] = [
    [48, 18, 59],
    [49, 21, 66],
    [50, 24, 74],
    [52, 27, 81],
    [53, 30, 88],
    [54, 33, 95],
    [55, 35, 101],
    [56, 38, 108],
    [57, 41, 114],
    [58, 44, 121],
    [59, 47, 127],
    [60, 50, 133],
    [60, 53, 139],
    [61, 55, 145],
    [62, 58, 150],
    [63, 61, 156],
    [64, 64, 161],
    [64, 67, 166],
    [65, 69, 171],
    [65, 72, 176],
    [66, 75, 181],
    [67, 78, 186],
    [67, 80, 190],
    [67, 83, 194],
    [68, 86, 199],
    [68, 88, 203],
    [69, 91, 206],
    [69, 94, 210],
    [69, 96, 214],
    [69, 99, 217],
    [70, 102, 221],
    [70, 104, 224],
    [70, 107, 227],
    [70, 109, 230],
    [70, 112, 232],
    [70, 115, 235],
    [70, 117, 237],
    [70, 120, 240],
    [70, 122, 242],
    [70, 125, 244],
    [70, 127, 246],
    [70, 130, 248],
    [69, 132, 249],
    [69, 135, 251],
    [69, 137, 252],
    [68, 140, 253],
    [67, 142, 253],
    [66, 145, 254],
    [65, 147, 254],
    [64, 150, 254],
    [63, 152, 254],
    [62, 155, 254],
    [60, 157, 253],
    [59, 160, 252],
    [57, 162, 252],
    [56, 165, 251],
    [54, 168, 249],
    [52, 170, 248],
    [51, 172, 246],
    [49, 175, 245],
    [47, 177, 243],
    [45, 180, 241],
    [43, 182, 239],
    [42, 185, 237],
    [40, 187, 235],
    [38, 189, 233],
    [37, 192, 230],
    [35, 194, 228],
    [33, 196, 225],
    [32, 198, 223],
    [30, 201, 220],
    [29, 203, 218],
    [28, 205, 215],
    [27, 207, 212],
    [26, 209, 210],
    [25, 211, 207],
    [24, 213, 204],
    [24, 215, 202],
    [23, 217, 199],
    [23, 218, 196],
    [23, 220, 194],
    [23, 222, 191],
    [24, 224, 189],
    [24, 225, 186],
    [25, 227, 184],
    [26, 228, 182],
    [27, 229, 180],
    [29, 231, 177],
    [30, 232, 175],
    [32, 233, 172],
    [34, 235, 169],
    [36, 236, 166],
    [39, 237, 163],
    [41, 238, 160],
    [44, 239, 157],
    [47, 240, 154],
    [50, 241, 151],
    [53, 243, 148],
    [56, 244, 145],
    [59, 244, 141],
    [63, 245, 138],
    [66, 246, 135],
    [70, 247, 131],
    [74, 248, 128],
    [77, 249, 124],
    [81, 249, 121],
    [85, 250, 118],
    [89, 251, 114],
    [93, 251, 111],
    [97, 252, 108],
    [101, 252, 104],
    [105, 253, 101],
    [109, 253, 98],
    [113, 253, 95],
    [116, 254, 92],
    [120, 254, 89],
    [124, 254, 86],
    [128, 254, 83],
    [132, 254, 80],
    [135, 254, 77],
    [139, 254, 75],
    [142, 254, 72],
    [146, 254, 70],
    [149, 254, 68],
    [152, 254, 66],
    [155, 253, 64],
    [158, 253, 62],
    [161, 252, 61],
    [164, 252, 59],
    [166, 251, 58],
    [169, 251, 57],
    [172, 250, 55],
    [174, 249, 55],
    [177, 248, 54],
    [179, 248, 53],
    [182, 247, 53],
    [185, 245, 52],
    [187, 244, 52],
    [190, 243, 52],
    [192, 242, 51],
    [195, 241, 51],
    [197, 239, 51],
    [200, 238, 51],
    [202, 237, 51],
    [205, 235, 52],
    [207, 234, 52],
    [209, 232, 52],
    [212, 231, 53],
    [214, 229, 53],
    [216, 227, 53],
    [218, 226, 54],
    [221, 224, 54],
    [223, 222, 54],
    [225, 220, 55],
    [227, 218, 55],
    [229, 216, 56],
    [231, 215, 56],
    [232, 213, 56],
    [234, 211, 57],
    [236, 209, 57],
    [237, 207, 57],
    [239, 205, 57],
    [240, 203, 58],
    [242, 200, 58],
    [243, 198, 58],
    [244, 196, 58],
    [246, 194, 58],
    [247, 192, 57],
    [248, 190, 57],
    [249, 188, 57],
    [249, 186, 56],
    [250, 183, 55],
    [251, 181, 55],
    [251, 179, 54],
    [252, 176, 53],
    [252, 174, 52],
    [253, 171, 51],
    [253, 169, 50],
    [253, 166, 49],
    [253, 163, 48],
    [254, 161, 47],
    [254, 158, 46],
    [254, 155, 45],
    [254, 152, 44],
    [253, 149, 43],
    [253, 146, 41],
    [253, 143, 40],
    [253, 140, 39],
    [252, 137, 38],
    [252, 134, 36],
    [251, 131, 35],
    [251, 128, 34],
    [250, 125, 32],
    [250, 122, 31],
    [249, 119, 30],
    [248, 116, 28],
    [247, 113, 27],
    [247, 110, 26],
    [246, 107, 24],
    [245, 104, 23],
    [244, 101, 22],
    [243, 99, 21],
    [242, 96, 20],
    [241, 93, 19],
    [239, 90, 17],
    [238, 88, 16],
    [237, 85, 15],
    [236, 82, 14],
    [234, 80, 13],
    [233, 77, 13],
    [232, 75, 12],
    [230, 73, 11],
    [229, 70, 10],
    [227, 68, 10],
    [226, 66, 9],
    [224, 64, 8],
    [222, 62, 8],
    [221, 60, 7],
    [219, 58, 7],
    [217, 56, 6],
    [215, 54, 6],
    [214, 52, 5],
    [212, 50, 5],
    [210, 48, 5],
    [208, 47, 4],
    [206, 45, 4],
    [203, 43, 3],
    [201, 41, 3],
    [199, 40, 3],
    [197, 38, 2],
    [195, 36, 2],
    [192, 35, 2],
    [190, 33, 2],
    [187, 31, 1],
    [185, 30, 1],
    [182, 28, 1],
    [180, 27, 1],
    [177, 25, 1],
    [174, 24, 1],
    [172, 22, 1],
    [169, 21, 1],
    [166, 20, 1],
    [163, 18, 1],
    [160, 17, 1],
    [157, 16, 1],
    [154, 14, 1],
    [151, 13, 1],
    [148, 12, 1],
    [145, 11, 1],
    [142, 10, 1],
    [139, 9, 1],
    [135, 8, 1],
    [132, 7, 1],
    [129, 6, 2],
    [125, 5, 2],
    [122, 4, 2],
];

// https://github.com/BIDS/colormap/blob/master/colormaps.py
const INFERNO: [[u8; 3]; 256] = [
    [0, 0, 3],
    [0, 0, 4],
    [0, 0, 6],
    [1, 0, 7],
    [1, 1, 9],
    [1, 1, 11],
    [2, 1, 14],
    [2, 2, 16],
    [3, 2, 18],
    [4, 3, 20],
    [4, 3, 22],
    [5, 4, 24],
    [6, 4, 27],
    [7, 5, 29],
    [8, 6, 31],
    [9, 6, 33],
    [10, 7, 35],
    [11, 7, 38],
    [13, 8, 40],
    [14, 8, 42],
    [15, 9, 45],
    [16, 9, 47],
    [18, 10, 50],
    [19, 10, 52],
    [20, 11, 54],
    [22, 11, 57],
    [23, 11, 59],
    [25, 11, 62],
    [26, 11, 64],
    [28, 12, 67],
    [29, 12, 69],
    [31, 12, 71],
    [32, 12, 74],
    [34, 11, 76],
    [36, 11, 78],
    [38, 11, 80],
    [39, 11, 82],
    [41, 11, 84],
    [43, 10, 86],
    [45, 10, 88],
    [46, 10, 90],
    [48, 10, 92],
    [50, 9, 93],
    [52, 9, 95],
    [53, 9, 96],
    [55, 9, 97],
    [57, 9, 98],
    [59, 9, 100],
    [60, 9, 101],
    [62, 9, 102],
    [64, 9, 102],
    [65, 9, 103],
    [67, 10, 104],
    [69, 10, 105],
    [70, 10, 105],
    [72, 11, 106],
    [74, 11, 106],
    [75, 12, 107],
    [77, 12, 107],
    [79, 13, 108],
    [80, 13, 108],
    [82, 14, 108],
    [83, 14, 109],
    [85, 15, 109],
    [87, 15, 109],
    [88, 16, 109],
    [90, 17, 109],
    [91, 17, 110],
    [93, 18, 110],
    [95, 18, 110],
    [96, 19, 110],
    [98, 20, 110],
    [99, 20, 110],
    [101, 21, 110],
    [102, 21, 110],
    [104, 22, 110],
    [106, 23, 110],
    [107, 23, 110],
    [109, 24, 110],
    [110, 24, 110],
    [112, 25, 110],
    [114, 25, 109],
    [115, 26, 109],
    [117, 27, 109],
    [118, 27, 109],
    [120, 28, 109],
    [122, 28, 109],
    [123, 29, 108],
    [125, 29, 108],
    [126, 30, 108],
    [128, 31, 107],
    [129, 31, 107],
    [131, 32, 107],
    [133, 32, 106],
    [134, 33, 106],
    [136, 33, 106],
    [137, 34, 105],
    [139, 34, 105],
    [141, 35, 105],
    [142, 36, 104],
    [144, 36, 104],
    [145, 37, 103],
    [147, 37, 103],
    [149, 38, 102],
    [150, 38, 102],
    [152, 39, 101],
    [153, 40, 100],
    [155, 40, 100],
    [156, 41, 99],
    [158, 41, 99],
    [160, 42, 98],
    [161, 43, 97],
    [163, 43, 97],
    [164, 44, 96],
    [166, 44, 95],
    [167, 45, 95],
    [169, 46, 94],
    [171, 46, 93],
    [172, 47, 92],
    [174, 48, 91],
    [175, 49, 91],
    [177, 49, 90],
    [178, 50, 89],
    [180, 51, 88],
    [181, 51, 87],
    [183, 52, 86],
    [184, 53, 86],
    [186, 54, 85],
    [187, 55, 84],
    [189, 55, 83],
    [190, 56, 82],
    [191, 57, 81],
    [193, 58, 80],
    [194, 59, 79],
    [196, 60, 78],
    [197, 61, 77],
    [199, 62, 76],
    [200, 62, 75],
    [201, 63, 74],
    [203, 64, 73],
    [204, 65, 72],
    [205, 66, 71],
    [207, 68, 70],
    [208, 69, 68],
    [209, 70, 67],
    [210, 71, 66],
    [212, 72, 65],
    [213, 73, 64],
    [214, 74, 63],
    [215, 75, 62],
    [217, 77, 61],
    [218, 78, 59],
    [219, 79, 58],
    [220, 80, 57],
    [221, 82, 56],
    [222, 83, 55],
    [223, 84, 54],
    [224, 86, 52],
    [226, 87, 51],
    [227, 88, 50],
    [228, 90, 49],
    [229, 91, 48],
    [230, 92, 46],
    [230, 94, 45],
    [231, 95, 44],
    [232, 97, 43],
    [233, 98, 42],
    [234, 100, 40],
    [235, 101, 39],
    [236, 103, 38],
    [237, 104, 37],
    [237, 106, 35],
    [238, 108, 34],
    [239, 109, 33],
    [240, 111, 31],
    [240, 112, 30],
    [241, 114, 29],
    [242, 116, 28],
    [242, 117, 26],
    [243, 119, 25],
    [243, 121, 24],
    [244, 122, 22],
    [245, 124, 21],
    [245, 126, 20],
    [246, 128, 18],
    [246, 129, 17],
    [247, 131, 16],
    [247, 133, 14],
    [248, 135, 13],
    [248, 136, 12],
    [248, 138, 11],
    [249, 140, 9],
    [249, 142, 8],
    [249, 144, 8],
    [250, 145, 7],
    [250, 147, 6],
    [250, 149, 6],
    [250, 151, 6],
    [251, 153, 6],
    [251, 155, 6],
    [251, 157, 6],
    [251, 158, 7],
    [251, 160, 7],
    [251, 162, 8],
    [251, 164, 10],
    [251, 166, 11],
    [251, 168, 13],
    [251, 170, 14],
    [251, 172, 16],
    [251, 174, 18],
    [251, 176, 20],
    [251, 177, 22],
    [251, 179, 24],
    [251, 181, 26],
    [251, 183, 28],
    [251, 185, 30],
    [250, 187, 33],
    [250, 189, 35],
    [250, 191, 37],
    [250, 193, 40],
    [249, 195, 42],
    [249, 197, 44],
    [249, 199, 47],
    [248, 201, 49],
    [248, 203, 52],
    [248, 205, 55],
    [247, 207, 58],
    [247, 209, 60],
    [246, 211, 63],
    [246, 213, 66],
    [245, 215, 69],
    [245, 217, 72],
    [244, 219, 75],
    [244, 220, 79],
    [243, 222, 82],
    [243, 224, 86],
    [243, 226, 89],
    [242, 228, 93],
    [242, 230, 96],
    [241, 232, 100],
    [241, 233, 104],
    [241, 235, 108],
    [241, 237, 112],
    [241, 238, 116],
    [241, 240, 121],
    [241, 242, 125],
    [242, 243, 129],
    [242, 244, 133],
    [243, 246, 137],
    [244, 247, 141],
    [245, 248, 145],
    [246, 250, 149],
    [247, 251, 153],
    [249, 252, 157],
    [250, 253, 160],
    [252, 254, 164],
];

const PLASMA: [[u8; 3]; 256] = [
    [12, 7, 134],
    [16, 7, 135],
    [19, 6, 137],
    [21, 6, 138],
    [24, 6, 139],
    [27, 6, 140],
    [29, 6, 141],
    [31, 5, 142],
    [33, 5, 143],
    [35, 5, 144],
    [37, 5, 145],
    [39, 5, 146],
    [41, 5, 147],
    [43, 5, 148],
    [45, 4, 148],
    [47, 4, 149],
    [49, 4, 150],
    [51, 4, 151],
    [52, 4, 152],
    [54, 4, 152],
    [56, 4, 153],
    [58, 4, 154],
    [59, 3, 154],
    [61, 3, 155],
    [63, 3, 156],
    [64, 3, 156],
    [66, 3, 157],
    [68, 3, 158],
    [69, 3, 158],
    [71, 2, 159],
    [73, 2, 159],
    [74, 2, 160],
    [76, 2, 161],
    [78, 2, 161],
    [79, 2, 162],
    [81, 1, 162],
    [82, 1, 163],
    [84, 1, 163],
    [86, 1, 163],
    [87, 1, 164],
    [89, 1, 164],
    [90, 0, 165],
    [92, 0, 165],
    [94, 0, 165],
    [95, 0, 166],
    [97, 0, 166],
    [98, 0, 166],
    [100, 0, 167],
    [101, 0, 167],
    [103, 0, 167],
    [104, 0, 167],
    [106, 0, 167],
    [108, 0, 168],
    [109, 0, 168],
    [111, 0, 168],
    [112, 0, 168],
    [114, 0, 168],
    [115, 0, 168],
    [117, 0, 168],
    [118, 1, 168],
    [120, 1, 168],
    [121, 1, 168],
    [123, 2, 168],
    [124, 2, 167],
    [126, 3, 167],
    [127, 3, 167],
    [129, 4, 167],
    [130, 4, 167],
    [132, 5, 166],
    [133, 6, 166],
    [134, 7, 166],
    [136, 7, 165],
    [137, 8, 165],
    [139, 9, 164],
    [140, 10, 164],
    [142, 12, 164],
    [143, 13, 163],
    [144, 14, 163],
    [146, 15, 162],
    [147, 16, 161],
    [149, 17, 161],
    [150, 18, 160],
    [151, 19, 160],
    [153, 20, 159],
    [154, 21, 158],
    [155, 23, 158],
    [157, 24, 157],
    [158, 25, 156],
    [159, 26, 155],
    [160, 27, 155],
    [162, 28, 154],
    [163, 29, 153],
    [164, 30, 152],
    [165, 31, 151],
    [167, 33, 151],
    [168, 34, 150],
    [169, 35, 149],
    [170, 36, 148],
    [172, 37, 147],
    [173, 38, 146],
    [174, 39, 145],
    [175, 40, 144],
    [176, 42, 143],
    [177, 43, 143],
    [178, 44, 142],
    [180, 45, 141],
    [181, 46, 140],
    [182, 47, 139],
    [183, 48, 138],
    [184, 50, 137],
    [185, 51, 136],
    [186, 52, 135],
    [187, 53, 134],
    [188, 54, 133],
    [189, 55, 132],
    [190, 56, 131],
    [191, 57, 130],
    [192, 59, 129],
    [193, 60, 128],
    [194, 61, 128],
    [195, 62, 127],
    [196, 63, 126],
    [197, 64, 125],
    [198, 65, 124],
    [199, 66, 123],
    [200, 68, 122],
    [201, 69, 121],
    [202, 70, 120],
    [203, 71, 119],
    [204, 72, 118],
    [205, 73, 117],
    [206, 74, 117],
    [207, 75, 116],
    [208, 77, 115],
    [209, 78, 114],
    [209, 79, 113],
    [210, 80, 112],
    [211, 81, 111],
    [212, 82, 110],
    [213, 83, 109],
    [214, 85, 109],
    [215, 86, 108],
    [215, 87, 107],
    [216, 88, 106],
    [217, 89, 105],
    [218, 90, 104],
    [219, 91, 103],
    [220, 93, 102],
    [220, 94, 102],
    [221, 95, 101],
    [222, 96, 100],
    [223, 97, 99],
    [223, 98, 98],
    [224, 100, 97],
    [225, 101, 96],
    [226, 102, 96],
    [227, 103, 95],
    [227, 104, 94],
    [228, 106, 93],
    [229, 107, 92],
    [229, 108, 91],
    [230, 109, 90],
    [231, 110, 90],
    [232, 112, 89],
    [232, 113, 88],
    [233, 114, 87],
    [234, 115, 86],
    [234, 116, 85],
    [235, 118, 84],
    [236, 119, 84],
    [236, 120, 83],
    [237, 121, 82],
    [237, 123, 81],
    [238, 124, 80],
    [239, 125, 79],
    [239, 126, 78],
    [240, 128, 77],
    [240, 129, 77],
    [241, 130, 76],
    [242, 132, 75],
    [242, 133, 74],
    [243, 134, 73],
    [243, 135, 72],
    [244, 137, 71],
    [244, 138, 71],
    [245, 139, 70],
    [245, 141, 69],
    [246, 142, 68],
    [246, 143, 67],
    [246, 145, 66],
    [247, 146, 65],
    [247, 147, 65],
    [248, 149, 64],
    [248, 150, 63],
    [248, 152, 62],
    [249, 153, 61],
    [249, 154, 60],
    [250, 156, 59],
    [250, 157, 58],
    [250, 159, 58],
    [250, 160, 57],
    [251, 162, 56],
    [251, 163, 55],
    [251, 164, 54],
    [252, 166, 53],
    [252, 167, 53],
    [252, 169, 52],
    [252, 170, 51],
    [252, 172, 50],
    [252, 173, 49],
    [253, 175, 49],
    [253, 176, 48],
    [253, 178, 47],
    [253, 179, 46],
    [253, 181, 45],
    [253, 182, 45],
    [253, 184, 44],
    [253, 185, 43],
    [253, 187, 43],
    [253, 188, 42],
    [253, 190, 41],
    [253, 192, 41],
    [253, 193, 40],
    [253, 195, 40],
    [253, 196, 39],
    [253, 198, 38],
    [252, 199, 38],
    [252, 201, 38],
    [252, 203, 37],
    [252, 204, 37],
    [252, 206, 37],
    [251, 208, 36],
    [251, 209, 36],
    [251, 211, 36],
    [250, 213, 36],
    [250, 214, 36],
    [250, 216, 36],
    [249, 217, 36],
    [249, 219, 36],
    [248, 221, 36],
    [248, 223, 36],
    [247, 224, 36],
    [247, 226, 37],
    [246, 228, 37],
    [246, 229, 37],
    [245, 231, 38],
    [245, 233, 38],
    [244, 234, 38],
    [243, 236, 38],
    [243, 238, 38],
    [242, 240, 38],
    [242, 241, 38],
    [241, 243, 38],
    [240, 245, 37],
    [240, 246, 35],
    [239, 248, 33],
];

const MAGMA: [[u8; 3]; 256] = [
    [0, 0, 3],
    [0, 0, 4],
    [0, 0, 6],
    [1, 0, 7],
    [1, 1, 9],
    [1, 1, 11],
    [2, 2, 13],
    [2, 2, 15],
    [3, 3, 17],
    [4, 3, 19],
    [4, 4, 21],
    [5, 4, 23],
    [6, 5, 25],
    [7, 5, 27],
    [8, 6, 29],
    [9, 7, 31],
    [10, 7, 34],
    [11, 8, 36],
    [12, 9, 38],
    [13, 10, 40],
    [14, 10, 42],
    [15, 11, 44],
    [16, 12, 47],
    [17, 12, 49],
    [18, 13, 51],
    [20, 13, 53],
    [21, 14, 56],
    [22, 14, 58],
    [23, 15, 60],
    [24, 15, 63],
    [26, 16, 65],
    [27, 16, 68],
    [28, 16, 70],
    [30, 16, 73],
    [31, 17, 75],
    [32, 17, 77],
    [34, 17, 80],
    [35, 17, 82],
    [37, 17, 85],
    [38, 17, 87],
    [40, 17, 89],
    [42, 17, 92],
    [43, 17, 94],
    [45, 16, 96],
    [47, 16, 98],
    [48, 16, 101],
    [50, 16, 103],
    [52, 16, 104],
    [53, 15, 106],
    [55, 15, 108],
    [57, 15, 110],
    [59, 15, 111],
    [60, 15, 113],
    [62, 15, 114],
    [64, 15, 115],
    [66, 15, 116],
    [67, 15, 117],
    [69, 15, 118],
    [71, 15, 119],
    [72, 16, 120],
    [74, 16, 121],
    [75, 16, 121],
    [77, 17, 122],
    [79, 17, 123],
    [80, 18, 123],
    [82, 18, 124],
    [83, 19, 124],
    [85, 19, 125],
    [87, 20, 125],
    [88, 21, 126],
    [90, 21, 126],
    [91, 22, 126],
    [93, 23, 126],
    [94, 23, 127],
    [96, 24, 127],
    [97, 24, 127],
    [99, 25, 127],
    [101, 26, 128],
    [102, 26, 128],
    [104, 27, 128],
    [105, 28, 128],
    [107, 28, 128],
    [108, 29, 128],
    [110, 30, 129],
    [111, 30, 129],
    [113, 31, 129],
    [115, 31, 129],
    [116, 32, 129],
    [118, 33, 129],
    [119, 33, 129],
    [121, 34, 129],
    [122, 34, 129],
    [124, 35, 129],
    [126, 36, 129],
    [127, 36, 129],
    [129, 37, 129],
    [130, 37, 129],
    [132, 38, 129],
    [133, 38, 129],
    [135, 39, 129],
    [137, 40, 129],
    [138, 40, 129],
    [140, 41, 128],
    [141, 41, 128],
    [143, 42, 128],
    [145, 42, 128],
    [146, 43, 128],
    [148, 43, 128],
    [149, 44, 128],
    [151, 44, 127],
    [153, 45, 127],
    [154, 45, 127],
    [156, 46, 127],
    [158, 46, 126],
    [159, 47, 126],
    [161, 47, 126],
    [163, 48, 126],
    [164, 48, 125],
    [166, 49, 125],
    [167, 49, 125],
    [169, 50, 124],
    [171, 51, 124],
    [172, 51, 123],
    [174, 52, 123],
    [176, 52, 123],
    [177, 53, 122],
    [179, 53, 122],
    [181, 54, 121],
    [182, 54, 121],
    [184, 55, 120],
    [185, 55, 120],
    [187, 56, 119],
    [189, 57, 119],
    [190, 57, 118],
    [192, 58, 117],
    [194, 58, 117],
    [195, 59, 116],
    [197, 60, 116],
    [198, 60, 115],
    [200, 61, 114],
    [202, 62, 114],
    [203, 62, 113],
    [205, 63, 112],
    [206, 64, 112],
    [208, 65, 111],
    [209, 66, 110],
    [211, 66, 109],
    [212, 67, 109],
    [214, 68, 108],
    [215, 69, 107],
    [217, 70, 106],
    [218, 71, 105],
    [220, 72, 105],
    [221, 73, 104],
    [222, 74, 103],
    [224, 75, 102],
    [225, 76, 102],
    [226, 77, 101],
    [228, 78, 100],
    [229, 80, 99],
    [230, 81, 98],
    [231, 82, 98],
    [232, 84, 97],
    [234, 85, 96],
    [235, 86, 96],
    [236, 88, 95],
    [237, 89, 95],
    [238, 91, 94],
    [238, 93, 93],
    [239, 94, 93],
    [240, 96, 93],
    [241, 97, 92],
    [242, 99, 92],
    [243, 101, 92],
    [243, 103, 91],
    [244, 104, 91],
    [245, 106, 91],
    [245, 108, 91],
    [246, 110, 91],
    [246, 112, 91],
    [247, 113, 91],
    [247, 115, 92],
    [248, 117, 92],
    [248, 119, 92],
    [249, 121, 92],
    [249, 123, 93],
    [249, 125, 93],
    [250, 127, 94],
    [250, 128, 94],
    [250, 130, 95],
    [251, 132, 96],
    [251, 134, 96],
    [251, 136, 97],
    [251, 138, 98],
    [252, 140, 99],
    [252, 142, 99],
    [252, 144, 100],
    [252, 146, 101],
    [252, 147, 102],
    [253, 149, 103],
    [253, 151, 104],
    [253, 153, 105],
    [253, 155, 106],
    [253, 157, 107],
    [253, 159, 108],
    [253, 161, 110],
    [253, 162, 111],
    [253, 164, 112],
    [254, 166, 113],
    [254, 168, 115],
    [254, 170, 116],
    [254, 172, 117],
    [254, 174, 118],
    [254, 175, 120],
    [254, 177, 121],
    [254, 179, 123],
    [254, 181, 124],
    [254, 183, 125],
    [254, 185, 127],
    [254, 187, 128],
    [254, 188, 130],
    [254, 190, 131],
    [254, 192, 133],
    [254, 194, 134],
    [254, 196, 136],
    [254, 198, 137],
    [254, 199, 139],
    [254, 201, 141],
    [254, 203, 142],
    [253, 205, 144],
    [253, 207, 146],
    [253, 209, 147],
    [253, 210, 149],
    [253, 212, 151],
    [253, 214, 152],
    [253, 216, 154],
    [253, 218, 156],
    [253, 220, 157],
    [253, 221, 159],
    [253, 223, 161],
    [253, 225, 163],
    [252, 227, 165],
    [252, 229, 166],
    [252, 230, 168],
    [252, 232, 170],
    [252, 234, 172],
    [252, 236, 174],
    [252, 238, 176],
    [252, 240, 177],
    [252, 241, 179],
    [252, 243, 181],
    [252, 245, 183],
    [251, 247, 185],
    [251, 249, 187],
    [251, 250, 189],
    [251, 252, 191],
];

const VIRIDIS: [[u8; 3]; 256] = [
    [68, 1, 84],
    [68, 2, 85],
    [68, 3, 87],
    [69, 5, 88],
    [69, 6, 90],
    [69, 8, 91],
    [70, 9, 92],
    [70, 11, 94],
    [70, 12, 95],
    [70, 14, 97],
    [71, 15, 98],
    [71, 17, 99],
    [71, 18, 101],
    [71, 20, 102],
    [71, 21, 103],
    [71, 22, 105],
    [71, 24, 106],
    [72, 25, 107],
    [72, 26, 108],
    [72, 28, 110],
    [72, 29, 111],
    [72, 30, 112],
    [72, 32, 113],
    [72, 33, 114],
    [72, 34, 115],
    [72, 35, 116],
    [71, 37, 117],
    [71, 38, 118],
    [71, 39, 119],
    [71, 40, 120],
    [71, 42, 121],
    [71, 43, 122],
    [71, 44, 123],
    [70, 45, 124],
    [70, 47, 124],
    [70, 48, 125],
    [70, 49, 126],
    [69, 50, 127],
    [69, 52, 127],
    [69, 53, 128],
    [69, 54, 129],
    [68, 55, 129],
    [68, 57, 130],
    [67, 58, 131],
    [67, 59, 131],
    [67, 60, 132],
    [66, 61, 132],
    [66, 62, 133],
    [66, 64, 133],
    [65, 65, 134],
    [65, 66, 134],
    [64, 67, 135],
    [64, 68, 135],
    [63, 69, 135],
    [63, 71, 136],
    [62, 72, 136],
    [62, 73, 137],
    [61, 74, 137],
    [61, 75, 137],
    [61, 76, 137],
    [60, 77, 138],
    [60, 78, 138],
    [59, 80, 138],
    [59, 81, 138],
    [58, 82, 139],
    [58, 83, 139],
    [57, 84, 139],
    [57, 85, 139],
    [56, 86, 139],
    [56, 87, 140],
    [55, 88, 140],
    [55, 89, 140],
    [54, 90, 140],
    [54, 91, 140],
    [53, 92, 140],
    [53, 93, 140],
    [52, 94, 141],
    [52, 95, 141],
    [51, 96, 141],
    [51, 97, 141],
    [50, 98, 141],
    [50, 99, 141],
    [49, 100, 141],
    [49, 101, 141],
    [49, 102, 141],
    [48, 103, 141],
    [48, 104, 141],
    [47, 105, 141],
    [47, 106, 141],
    [46, 107, 142],
    [46, 108, 142],
    [46, 109, 142],
    [45, 110, 142],
    [45, 111, 142],
    [44, 112, 142],
    [44, 113, 142],
    [44, 114, 142],
    [43, 115, 142],
    [43, 116, 142],
    [42, 117, 142],
    [42, 118, 142],
    [42, 119, 142],
    [41, 120, 142],
    [41, 121, 142],
    [40, 122, 142],
    [40, 122, 142],
    [40, 123, 142],
    [39, 124, 142],
    [39, 125, 142],
    [39, 126, 142],
    [38, 127, 142],
    [38, 128, 142],
    [38, 129, 142],
    [37, 130, 142],
    [37, 131, 141],
    [36, 132, 141],
    [36, 133, 141],
    [36, 134, 141],
    [35, 135, 141],
    [35, 136, 141],
    [35, 137, 141],
    [34, 137, 141],
    [34, 138, 141],
    [34, 139, 141],
    [33, 140, 141],
    [33, 141, 140],
    [33, 142, 140],
    [32, 143, 140],
    [32, 144, 140],
    [32, 145, 140],
    [31, 146, 140],
    [31, 147, 139],
    [31, 148, 139],
    [31, 149, 139],
    [31, 150, 139],
    [30, 151, 138],
    [30, 152, 138],
    [30, 153, 138],
    [30, 153, 138],
    [30, 154, 137],
    [30, 155, 137],
    [30, 156, 137],
    [30, 157, 136],
    [30, 158, 136],
    [30, 159, 136],
    [30, 160, 135],
    [31, 161, 135],
    [31, 162, 134],
    [31, 163, 134],
    [32, 164, 133],
    [32, 165, 133],
    [33, 166, 133],
    [33, 167, 132],
    [34, 167, 132],
    [35, 168, 131],
    [35, 169, 130],
    [36, 170, 130],
    [37, 171, 129],
    [38, 172, 129],
    [39, 173, 128],
    [40, 174, 127],
    [41, 175, 127],
    [42, 176, 126],
    [43, 177, 125],
    [44, 177, 125],
    [46, 178, 124],
    [47, 179, 123],
    [48, 180, 122],
    [50, 181, 122],
    [51, 182, 121],
    [53, 183, 120],
    [54, 184, 119],
    [56, 185, 118],
    [57, 185, 118],
    [59, 186, 117],
    [61, 187, 116],
    [62, 188, 115],
    [64, 189, 114],
    [66, 190, 113],
    [68, 190, 112],
    [69, 191, 111],
    [71, 192, 110],
    [73, 193, 109],
    [75, 194, 108],
    [77, 194, 107],
    [79, 195, 105],
    [81, 196, 104],
    [83, 197, 103],
    [85, 198, 102],
    [87, 198, 101],
    [89, 199, 100],
    [91, 200, 98],
    [94, 201, 97],
    [96, 201, 96],
    [98, 202, 95],
    [100, 203, 93],
    [103, 204, 92],
    [105, 204, 91],
    [107, 205, 89],
    [109, 206, 88],
    [112, 206, 86],
    [114, 207, 85],
    [116, 208, 84],
    [119, 208, 82],
    [121, 209, 81],
    [124, 210, 79],
    [126, 210, 78],
    [129, 211, 76],
    [131, 211, 75],
    [134, 212, 73],
    [136, 213, 71],
    [139, 213, 70],
    [141, 214, 68],
    [144, 214, 67],
    [146, 215, 65],
    [149, 215, 63],
    [151, 216, 62],
    [154, 216, 60],
    [157, 217, 58],
    [159, 217, 56],
    [162, 218, 55],
    [165, 218, 53],
    [167, 219, 51],
    [170, 219, 50],
    [173, 220, 48],
    [175, 220, 46],
    [178, 221, 44],
    [181, 221, 43],
    [183, 221, 41],
    [186, 222, 39],
    [189, 222, 38],
    [191, 223, 36],
    [194, 223, 34],
    [197, 223, 33],
    [199, 224, 31],
    [202, 224, 30],
    [205, 224, 29],
    [207, 225, 28],
    [210, 225, 27],
    [212, 225, 26],
    [215, 226, 25],
    [218, 226, 24],
    [220, 226, 24],
    [223, 227, 24],
    [225, 227, 24],
    [228, 227, 24],
    [231, 228, 25],
    [233, 228, 25],
    [236, 228, 26],
    [238, 229, 27],
    [241, 229, 28],
    [243, 229, 30],
    [246, 230, 31],
    [248, 230, 33],
    [250, 230, 34],
    [253, 231, 36],
];

const BW: [[u8; 3]; 256] = [
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [6, 6, 6],
    [12, 12, 12],
    [19, 19, 19],
    [25, 25, 25],
    [31, 31, 31],
    [38, 38, 38],
    [44, 44, 44],
    [51, 51, 51],
    [57, 57, 57],
    [63, 63, 63],
    [70, 70, 70],
    [76, 76, 76],
    [82, 82, 82],
    [89, 89, 89],
    [95, 95, 95],
    [102, 102, 102],
    [108, 108, 108],
    [114, 114, 114],
    [121, 121, 121],
    [127, 127, 127],
    [133, 133, 133],
    [140, 140, 140],
    [146, 146, 146],
    [153, 153, 153],
    [159, 159, 159],
    [165, 165, 165],
    [172, 172, 172],
    [178, 178, 178],
    [184, 184, 184],
    [191, 191, 191],
    [197, 197, 197],
    [204, 204, 204],
    [210, 210, 210],
    [216, 216, 216],
    [223, 223, 223],
    [229, 229, 229],
    [235, 235, 235],
    [242, 242, 242],
    [248, 248, 248],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
    [255, 255, 255],
];
