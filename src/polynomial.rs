use std::error::Error;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct Polynomial(Vec<f64>);

impl Default for Polynomial {
    fn default() -> Self {
        Self(vec![0.0])
    }
}

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "<empty>");
        }

        let first = self.0[self.degree()];

        if self.degree() == 0 {
            write!(f, "{}", first)?;
        } else if first == 1.0 {
            write!(f, "x^{}", self.degree())?;
        } else if first == -1.0 {
            write!(f, "-x^{}", self.degree())?;
        } else {
            write!(f, "{}x^{}", first, self.degree())?;
        }

        for (i, coeff) in self.0.iter().take(self.0.len() - 1).enumerate().rev() {
            if coeff == &0.0 {
                continue;
            }

            if i == 0 {
                write!(
                    f,
                    " {} {}",
                    if coeff < &0.0 { '-' } else { '+' },
                    coeff.abs()
                )?;
            } else if coeff.abs() == 1.0 {
                write!(f, " {} x^{}", if coeff < &0.0 { '-' } else { '+' }, i)?;
            } else {
                write!(
                    f,
                    " {} {}x^{}",
                    if coeff < &0.0 { '-' } else { '+' },
                    coeff.abs(),
                    i
                )?;
            }
        }
        Ok(())
    }
}

impl std::ops::Neg for Polynomial {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.iter().map(|a| -*a).collect())
    }
}

impl std::ops::Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let len = std::cmp::max(self.0.len(), rhs.0.len());

        let mut lhs = self.0;
        let mut rhs = rhs.0;

        lhs.resize(len, 0.0);
        rhs.resize(len, 0.0);

        Self(
            lhs.iter()
                .zip(rhs)
                .map(|(a, b)| a + b)
                .collect::<Vec<f64>>(),
        )
    }
}

impl std::ops::Add<f64> for Polynomial {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        self + Self(vec![rhs])
    }
}

impl std::ops::AddAssign for Polynomial {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl std::ops::AddAssign<f64> for Polynomial {
    fn add_assign(&mut self, rhs: f64) {
        *self += Self(vec![rhs]);
    }
}

impl std::ops::Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl std::ops::Sub<f64> for Polynomial {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        self - Self(vec![rhs])
    }
}

impl std::ops::SubAssign for Polynomial {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() + -rhs;
    }
}

impl std::ops::SubAssign<f64> for Polynomial {
    fn sub_assign(&mut self, rhs: f64) {
        *self -= Self(vec![rhs]);
    }
}

impl std::ops::Mul for Polynomial {
    type Output = Self;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut item = Self(Vec::<f64>::default());
        let degree = self.degree() + rhs.degree();

        for i in 0..=degree {
            let mut coeff = 0.0;
            for l in 0..=self.degree() {
                for r in 0..=rhs.degree() {
                    if l + r == i {
                        coeff += self.0[l] * rhs.0[r];
                    }
                }
            }
            item.0.push(coeff);
        }

        item
    }
}

impl std::ops::Mul<f64> for Polynomial {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * Self(vec![rhs])
    }
}

impl std::ops::MulAssign for Polynomial {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl std::ops::MulAssign<f64> for Polynomial {
    fn mul_assign(&mut self, rhs: f64) {
        *self *= Self(vec![rhs])
    }
}

#[allow(dead_code, unused_variables)]
// constructors
impl Polynomial {
    /// формирование многочлена в формате `[a0, a1, a2, ..., an]`, где n - степень многочлена, `а` -
    /// коэффициент при соответствующих одночленах
    pub fn from_coeffs<T>(vec: Vec<T>) -> Result<Self, Box<dyn Error>>
    where
        T: Into<f64>,
    {
        let mut item = Self(vec.into_iter().map(|a| a.into()).collect());
        item.cut_last_zeroes()?;
        Ok(item)
    }

    /// формирование многочлена в формате `[(a, i), ...]`, где `a` - коэффициент при одночлене, `i` - его степень
    pub fn from_pairs<T>(pairs: Vec<(T, usize)>) -> Result<Self, Box<dyn Error>>
    where
        T: Into<f64> + Copy,
    {
        // поиск одинаковых значений степени одночлена, что является ошибкой
        let mut uniques: Vec<usize> = pairs.iter().map(|(_, degree)| degree).cloned().collect();
        uniques.dedup();
        if uniques.len() != pairs.len() {
            Err("Degree collision")?
        }
        // поиск степени будущего многочлена
        let max_degree = pairs
            .iter()
            .max_by_key(|(_, degree)| *degree)
            .ok_or("Cannot find max degree")?
            .1;
        // формирование объекта
        let mut item = Self(Vec::<f64>::default());
        for i in 0..=max_degree {
            match pairs.iter().position(|(_, degree)| degree == &i) {
                Some(index) => item.0.push(pairs[index].0.into()),
                None => item.0.push(0.0),
            }
        }

        Ok(item)
    }

    /// формирование многочлена по интерполяционной формуле Лагранжа, входные данные в формате:
    /// `[(x0, f(x0)), (x1, f(x1)), ..., (xn, f(xn))]`
    pub fn lagrange<T>(pairs: Vec<(T, T)>) -> Result<Self, Box<dyn Error>>
    where
        T: Into<f64> + Copy,
    {
        let mut item = Self::default();

        for i in 0..pairs.len() {
            let xi: f64 = pairs[i].0.into();
            let fi: f64 = pairs[i].1.into();

            let mut li = Self::from_coeffs(vec![1])?;

            for (j, (xj, fj)) in pairs.iter().enumerate() {
                if i != j {
                    let xj: f64 = (*xj).into();
                    li *= Self::from_coeffs(vec![-xj, 1.0])? * (1.0 / (xi - xj));
                }
            }
            item += li * fi;
        }
        item.cut_last_zeroes()?;
        Ok(item)
    }
}

#[allow(dead_code)]
// public
impl Polynomial {
    pub fn degree(&self) -> usize {
        self.0.len() - 1
    }

    pub fn coeff(&self, degree: usize) -> Option<f64> {
        if degree > self.degree() {
            None
        } else {
            Some(self.0[degree])
        }
    }

    /// `f(x)` - вычисление значения полинома в точке
    pub fn f(&self, x: f64) -> f64 {
        let mut item = 0.0;
        for (degree, coeff) in self.0.iter().enumerate() {
            item += coeff * x.powi(degree as i32);
        }
        item
    }
}

// private
impl Polynomial {
    fn cut_last_zeroes(&mut self) -> Result<(), Box<dyn Error>> {
        if self.0.last().ok_or("Must not be an empty vector")? == &0.0 {
            let mut new_len = self.0.len()
                - self
                    .0
                    .iter()
                    .rev()
                    .position(|&a| a != 0.0)
                    .unwrap_or(self.0.len());
            if new_len == 0 {
                new_len = 1
            }
            self.0.resize(new_len, 0.0);
        }
        Ok(())
    }
}

#[test]
#[should_panic]
fn degree_collision() {
    let _a = Polynomial::from_pairs(vec![(0, 1), (2, 1)]).unwrap();
}

#[test]
#[should_panic]
fn empty_vector() {
    let _a = Polynomial::from_coeffs(Vec::<f64>::default()).unwrap();
}

#[test]
fn lagrange() {
    let a = Polynomial::lagrange(vec![(0, 0), (1, 1), (2, 4), (3, 9)]).unwrap();
    let b = Polynomial::from_pairs(vec![(1, 2)]).unwrap();
    assert_eq!(a, b)
}

#[test]
fn arith() {
    let a = Polynomial::from_pairs(vec![(1, 127), (2, 64), (1, 0)]).unwrap();
    let b = Polynomial::from_pairs(vec![(-1, 127), (-2, 64), (-1, 0)]).unwrap();
    assert_eq!(b, -a);

    let a = Polynomial::from_pairs(vec![(1, 127), (2, 64), (1, 0)]).unwrap();
    let b = Polynomial::from_pairs(vec![(23, 99), (21, 64), (-9, 0)]).unwrap();
    let c = Polynomial::from_pairs(vec![(1, 127), (23, 99), (23, 64), (-8, 0)]).unwrap();
    assert_eq!(a + b, c);

    let a = Polynomial::from_pairs(vec![(1, 127), (2, 64), (1, 0)]).unwrap();
    let b = Polynomial::from_pairs(vec![(21, 64), (23, 99), (-9, 0)]).unwrap();
    let c = Polynomial::from_pairs(vec![(-23, 99), (1, 127), (-19, 64), (10, 0)]).unwrap();
    assert_eq!(a - b, c);

    let a = Polynomial::from_pairs(vec![(1, 127), (2, 64), (1, 0)]).unwrap();
    let b = Polynomial::from_pairs(vec![(23, 99), (21, 64), (-9, 0)]).unwrap();
    let c = Polynomial::from_pairs(vec![
        (23, 226),
        (21, 191),
        (46, 163),
        (42, 128),
        (-9, 127),
        (23, 99),
        (3, 64),
        (-9, 0),
    ])
    .unwrap();
    assert_eq!(a * b, c);
}

#[test]
fn calc_x() {
    let poly = Polynomial::from_pairs(vec![(1, 2)]).unwrap();
    assert_eq!(4.0, poly.f(2.0));

    let poly = Polynomial::from_pairs(vec![(1, 5), (-1, 0)]).unwrap();
    assert_eq!(0.0, poly.f(1.0));
    assert_eq!(31.0, poly.f(2.0));
}
