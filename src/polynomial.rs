use std::error::Error;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
struct Polynomial(Vec<f64>);

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

#[allow(dead_code, unused_variables)]
// constructors
impl Polynomial {
    pub fn from_pairs<T>(mut pairs: Vec<(usize, T)>) -> Result<Self, Box<dyn Error>>
    where
        T: Into<f64> + Copy,
    {
        let mut uniques: Vec<usize> = pairs.iter().map(|(degree, _)| degree).cloned().collect();
        uniques.dedup();
        if uniques.len() != pairs.len() {
            Err("Degree collision")?
        }

        pairs.sort_by_key(|(degree, _)| *degree);
        let max_degree = pairs
            .iter()
            .max_by_key(|(degree, _)| *degree)
            .ok_or("Cannot find max degree")?
            .0;

        let mut item = Self::default();
        for i in 0..=max_degree {
            match pairs.iter().position(|(degree, _)| degree == &i) {
                Some(index) => item.0.push(pairs[index].1.into()),
                None => item.0.push(0.0),
            }
        }

        Ok(item)
    }
}

#[allow(dead_code)]
// public
impl Polynomial {
    pub fn degree(&self) -> usize {
        self.0.len() - 1
    }
}

// private
impl Polynomial {}


#[test]
#[should_panic]
fn degree_collision() {
    let _a = Polynomial::from_pairs(vec![(1, 0), (1, 2)]).unwrap();
}

#[test]
fn arith() {
    let a = Polynomial::from_pairs(vec![(127, 1), (64, 2), (0, 1)]).unwrap();
    let b = Polynomial::from_pairs(vec![(99, 23), (64, 21), (0, -9)]).unwrap();
    let c = Polynomial::from_pairs(vec![(127, 1), (99, 23), (64, 23), (0, -8)]).unwrap();
    assert_eq!(a + b, c);

    let a = Polynomial::from_pairs(vec![(127, 1), (64, 2), (0, 1)]).unwrap();
    let b = Polynomial::from_pairs(vec![(127, -1), (64, -2), (0, -1)]).unwrap();
    assert_eq!(b, -a);
}
