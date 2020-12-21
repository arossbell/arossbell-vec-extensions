/*
 * arossbell's Rust Vec extensions.
 * Some things aren't to my liking in
 *  Rust's current Vec functions ...
 *  they prevent me from writing huge
 *  one-liners.
 * This code will extend Vec types
 *  to enable me to write huge
 *  one-liners.
 *
 * Example Addition:
 *  `use arossbell_vector_extensions::*;`
 *
 * Anti-Frivolous Lawsuit Statement:
 * This code is released under the MIT license,
 *  authored by Adam R Bell (github/arossbell)
 *  in AD 2020/2021.
 *
 */

use std::vec::Vec;

pub trait ArossbellVecExt<T>
{
    fn prepend_value(
        &self,
        val: &T,
    ) -> Vec<T>;

    fn postpend_value(
        &self,
        val: &T,
    ) -> Vec<T>;

}

impl<T: Copy> ArossbellVecExt<T> for Vec<T>
{
        fn prepend_value(
            &self,
            val: &T,
        ) -> Vec<T>
        {
            let mut tmpvec: Vec<T> = vec![*val];
            tmpvec.extend((*self).iter());
            tmpvec
        }

        fn postpend_value(
            &self,
            val: &T,
        ) -> Vec<T>
        {
            let mut tmpvec: Vec<T> = self.to_vec();
            tmpvec.extend(vec![*val].iter());
            tmpvec
        }

}


#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn prepend_value_test()
    {
        assert_eq!(
            vec![2,3,4].prepend_value(&1),
            vec![1,2,3,4],
        );
    }
    #[test]
    fn postpend_value_test()
    {
        assert_eq!(
            vec![2,3,4].postpend_value(&1),
            vec![2,3,4,1],
        );
    }
    #[test]
    fn another_postpend_test()
    {
        // This type of action was kind of the purpose
        //  of these short-hand functions.
        assert_eq!(
            vec![1,2,3]
                .postpend_value(&4)
                .iter()
                .map(
                    |a|
                    *a * *a
                )
                .collect::<Vec<u16>>(),
            vec![1,4,9,16]
        );
    }
}
