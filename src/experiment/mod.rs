trait Identity<Z> {
    fn identity(value: Z) -> Self;
}

impl<T: Sized> Identity<T> for T {
    fn identity(value: Self) -> Self {
        value
    }
}

trait RefIdentity<Z: ?Sized> {
    fn ref_identity(value: &Z) -> &Self;
}

impl<T: ?Sized> RefIdentity<T> for T {
    fn ref_identity(value: &Self) -> &Self {
        value
    }
}

trait RefFromRef<T> {
    fn ref_from_ref(value_ref: &T) -> &Self;
}

trait MutIdentity<Z: ?Sized> {
    fn mut_identity(value: &mut Z) -> &mut Self;
}

impl<T: ?Sized> MutIdentity<T> for T {
    fn mut_identity(value: &mut Self) -> &mut Self {
        value
    }
}

trait MutFromMut<T> {
    fn mut_from_mut(value_ref: &mut T) -> &mut Self;
}

trait ToArray<'b, Z, const N: usize> {
    fn to_array<'a: 'b, T: Identity<Z>>(&'a self) -> [T; N];
}

impl<'b, A: 'b, B: 'b, Z> ToArray<'b, Z, 2> for (A, B)
where
    Z: From<&'b A> + From<&'b B> + 'b,
{
    fn to_array<'a: 'b, T: Identity<Z>>(&'a self) -> [T; 2] {
        [T::identity(Z::from(&self.0)), T::identity(Z::from(&self.1))]
    }
}

trait AsIter<'b, Z: ?Sized> {
    type Iter<'a, T>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a + ?Sized,
        'b: 'a;

    fn as_iter<'a, T: RefIdentity<Z> + ?Sized>(&'a self) -> Self::Iter<'a, T>
    where
        'b: 'a;
}

impl<'b, A, B, Z> AsIter<'b, Z> for (A, B)
where
    Z: RefFromRef<A> + RefFromRef<B> + 'b + ?Sized,
{
    type Iter<'a, T> = core::array::IntoIter<&'a T, 2>
    where
        Self: 'a, T: 'a + ?Sized, 'b: 'a;

    fn as_iter<'a, T: RefIdentity<Z> + ?Sized>(&'a self) -> Self::Iter<'a, T>
    where
        'b: 'a,
    {
        [
            T::ref_identity(Z::ref_from_ref(&self.0)),
            T::ref_identity(Z::ref_from_ref(&self.1)),
        ]
        .into_iter()
    }
}

trait AsIterMut<'b, Z: ?Sized> {
    type Iter<'a, T>: Iterator<Item = &'a mut T>
    where
        Self: 'a,
        T: 'a + ?Sized,
        'b: 'a;

    fn as_iter_mut<'a, T: MutIdentity<Z> + ?Sized>(&'a mut self) -> Self::Iter<'a, T>
    where
        'b: 'a;
}

impl<'b, A, B, Z> AsIterMut<'b, Z> for (A, B)
where
    Z: MutFromMut<A> + MutFromMut<B> + 'b + ?Sized,
{
    type Iter<'a, T> = core::array::IntoIter<&'a mut T, 2>
    where
        Self: 'a, T: 'a + ?Sized, 'b: 'a;

    fn as_iter_mut<'a, T: MutIdentity<Z> + ?Sized>(&'a mut self) -> Self::Iter<'a, T>
    where
        'b: 'a,
    {
        [
            T::mut_identity(Z::mut_from_mut(&mut self.0)),
            T::mut_identity(Z::mut_from_mut(&mut self.1)),
        ]
        .into_iter()
    }
}

trait ToArrayMut<'b, Z, const N: usize> {
    fn to_array_mut<'a: 'b, T: Identity<Z>>(&'a mut self) -> [T; N];
}

impl<'b, A: 'b, B: 'b, Z> ToArrayMut<'b, Z, 2> for (A, B)
where
    Z: From<&'b mut A> + From<&'b mut B> + 'b,
{
    fn to_array_mut<'a: 'b, T: Identity<Z>>(&'a mut self) -> [T; 2] {
        [
            T::identity(Z::from(&mut self.0)),
            T::identity(Z::from(&mut self.1)),
        ]
    }
}

trait ToArrayOwned<Z, const N: usize> {
    fn to_array_owned<T: Identity<Z>>(self) -> [T; N];
}

impl<'b, A: 'b, B: 'b, Z> ToArrayOwned<Z, 2> for (A, B)
where
    Z: From<A> + From<B> + 'b,
{
    fn to_array_owned<T: Identity<Z>>(self) -> [T; 2] {
        [T::identity(Z::from(self.0)), T::identity(Z::from(self.1))]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_array() {
        for x in (1_u8, 2_i8).to_array_mut::<&mut dyn Example>() {
            dbg!(x.add_one());
        }

        for x in (1_u8, 2_i8).to_array_owned::<Box<dyn Example>>() {
            dbg!(x.value());
        }

        let values: Vec<_> = (1_u8, 2_i8)
            .to_array::<&dyn Example>()
            .into_iter()
            .map(|x| x.value())
            .collect();

        assert_eq!(values, [1, 2]);
    }

    #[test]
    fn as_iter() {
        let values: Vec<_> = (1_u8, 2_i8)
            .as_iter::<dyn Example>()
            .map(|x| x.value())
            .collect();

        assert_eq!(values, [1, 2]);
    }

    #[test]
    fn as_iter_mut() {
        let mut data = (1_u8, 2_i8);

        for x in data.as_iter_mut::<dyn Example>() {
            x.add_one();
        }

        let values: Vec<_> = data.as_iter::<dyn Example>().map(|x| x.value()).collect();

        assert_eq!(values, [2, 3]);
    }

    #[test]
    fn example_list_tuple() {
        let values: Vec<_> = (1_u8, 2_i8).iter_examples().map(|x| x.value()).collect();

        assert_eq!(values, [1, 2]);
    }

    #[test]
    fn example_list_array() {
        let values: Vec<_> = [1_u8, 2_u8].iter_examples().map(|x| x.value()).collect();

        assert_eq!(values, [1, 2]);
    }

    trait Example {
        fn value(&self) -> i32;
        fn add_one(&mut self);
    }

    impl<T: Example + 'static> RefFromRef<T> for dyn Example {
        fn ref_from_ref(value_ref: &T) -> &Self {
            value_ref
        }
    }

    impl<T: Example + 'static> MutFromMut<T> for dyn Example {
        fn mut_from_mut(value_ref: &mut T) -> &mut Self {
            value_ref
        }
    }

    impl<'a, T: Example> From<&'a T> for &'a dyn Example {
        fn from(value: &'a T) -> Self {
            value
        }
    }

    impl<'a, T: Example> From<&'a mut T> for &'a mut dyn Example {
        fn from(value: &'a mut T) -> Self {
            value
        }
    }

    impl<T: Example + 'static> From<T> for Box<dyn Example> {
        fn from(value: T) -> Self {
            Box::new(value)
        }
    }

    trait ExampleList {
        type Item: Example + ?Sized;
        type IntoIter<'a>: Iterator<Item = &'a Self::Item>
        where
            Self: 'a;

        fn iter_examples<'a>(&'a self) -> Self::IntoIter<'a>;
    }

    impl<T> ExampleList for T
    where
        T: AsIter<'static, dyn Example>,
    {
        type Item = dyn Example;

        type IntoIter<'a> = T::Iter<'a, dyn Example + 'static>
        where
            Self: 'a;

        fn iter_examples<'a>(&'a self) -> Self::IntoIter<'a> {
            self.as_iter()
        }
    }

    impl<T: Example, const N: usize> ExampleList for [T; N] {
        type Item = T;

        type IntoIter<'a> = core::slice::Iter<'a, T> where T: 'a;

        fn iter_examples<'a>(&'a self) -> Self::IntoIter<'a> {
            self.iter()
        }
    }

    impl Example for u8 {
        fn value(&self) -> i32 {
            *self as i32
        }

        fn add_one(&mut self) {
            *self += 1;
        }
    }

    impl Example for i8 {
        fn value(&self) -> i32 {
            *self as i32
        }

        fn add_one(&mut self) {
            *self += 1;
        }
    }
}
