pub struct Heap {
    original: Vec<u32>,
}

impl Heap {
    pub fn perms(original: Vec<u32>) -> Vec<Vec<u32>> {
        let size = original.len();
        let mut result: Vec<Vec<u32>> = vec![];
        Heap::permutate(&mut original.clone(), size, &mut result);
        result
    }

    fn permutate(original: &mut Vec<u32>, k: usize, result: &mut Vec<Vec<u32>>) {
        if k == 1 {
            result.push(original.clone());
        } else {
            for i in 0..(k - 1) {
                Heap::permutate(original, k - 1, result);
                if k % 2 == 0 {
                    Heap::swap(i, k - 1, original)
                } else {
                    Heap::swap(0, k - 1, original)
                };
            }
            Heap::permutate(original, k - 1, result);
        }
    }

    fn swap(p1: usize, p2: usize, original: &mut Vec<u32>) -> &mut Vec<u32> {
        let tmp = original[p1];
        original[p1] = original[p2];
        original[p2] = tmp;
        original
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generates_permutations_for_1() {
        let h = Heap::perms(vec![9]);
        let expected: Vec<Vec<u32>> = vec![vec![9]];
        assert_eq!(h.len(), 1);
        assert_eq!(h, expected);
    }

    #[test]
    fn generates_permutations_for_2() {
        let h = Heap::perms(vec![1, 2]);
        let expected: Vec<Vec<u32>> = vec![vec![1, 2], vec![2, 1]];
        assert_eq!(h.len(), 2);
        assert_eq!(h, expected);
    }

    #[test]
    fn generates_permutations_for_3() {
        let h = Heap::perms(vec![1, 2, 3]);
        assert_eq!(h.len(), 6);
        println!("{:?}", h);
        for (i, p) in h.iter().enumerate() {
            assert_eq!(h.iter().position(|x| x == p).unwrap(), i);
        }
    }

    #[test]
    fn generates_permutations_for_4() {
        let phase: Vec<u32> = (0..=4).collect();
        let h = Heap::perms(phase);
        assert_eq!(h.len(), 120);
        for (i, p) in h.iter().enumerate() {
            assert_eq!(h.iter().position(|x| x == p).unwrap(), i);
        }
    }

}
