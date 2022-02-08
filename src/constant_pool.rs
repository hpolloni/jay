
#[derive(Debug)]
pub enum Constant {
    Class(usize),
    Utf8(String),
    NameAndType(usize, usize),
    FieldRef(usize, usize),
    MethodRef(usize, usize),
    Double(f64),
    Integer(i32),
    Float(f32),
    Long(i64),
}

#[derive(Debug)]
pub struct ConstantPool {
    entries: Vec<Constant>
}

impl ConstantPool {
    pub fn new() -> Self {
        let entries = Vec::new();
        Self { entries }
    }

    pub fn push(&mut self, constant: Constant) {
        self.entries.push(constant)
    }

    pub fn get_utf8_at(&self, index: usize) -> Option<&String> {
        self.entries
            .get(index)
            .map(|utf8| match utf8{ Constant::Utf8(s) => Some(s), _ => None})
            .flatten()
    }

    pub fn get_class_name(&self, class_index: usize) -> &String {
        let class = self.get_class_at(class_index).unwrap();
        if let Constant::Class(name_index) = class {
            let name = self.get_utf8_at(*name_index).unwrap();
            return name;
        }
        panic!("Invalid constant pool");
    }

    fn get_class_at(&self, index: usize) -> Option<&Constant> {
        self.entries.get(index).filter(|c| matches!(c, Constant::Class(_)))
    }

    fn get_name_and_type(&self, index: usize) -> Option<&Constant> {
        self.entries.get(index).filter(|c| matches!(c, Constant::NameAndType(_,_)))
    }

    fn get_field_ref(&self, index: usize) -> Option<&Constant> {
        self.entries.get(index).filter(|c| matches!(c, Constant::FieldRef(_,_)))
    }

    fn get_method_ref(&self, index: usize) -> Option<&Constant> {
        self.entries.get(index).filter(|c| matches!(c, Constant::MethodRef(_,_)))
    }

    fn get_double(&self, index: usize) -> Option<&Constant> {
        self.entries.get(index).filter(|c| matches!(c, Constant::Double(_)))
    }

    fn get_integer(&self, index: usize) -> Option<&Constant> {
        self.entries.get(index).filter(|c| matches!(c, Constant::Integer(_)))
    }

    fn get_float(&self, index: usize) -> Option<&Constant> {
        self.entries.get(index).filter(|c| matches!(c, Constant::Float(_)))
    }

    fn get_long(&self, index: usize) -> Option<&Constant> {
        self.entries.get(index).filter(|c| matches!(c, Constant::Long(_)))
    }


    fn validate_constant(&self, constant: &Constant) -> bool {
        match constant {
            Constant::Class(n) => self.get_utf8_at(*n).is_some(),
            Constant::NameAndType(n, d) => {
                self.get_utf8_at(*n).is_some() &&
                self.get_utf8_at(*d).is_some()
            },
            Constant::FieldRef(c, nt) => {
                self.get_class_at(*c).is_some() &&
                self.get_name_and_type(*nt).is_some()
            },
            Constant::MethodRef(c, nt) => {
                self.get_class_at(*c).is_some() &&
                self.get_name_and_type(*nt).is_some()
            },
            Constant::Utf8(_) => true,
            Constant::Double(_) => true,
            Constant::Integer(_) => true,
            Constant::Float(_) => true,
            Constant::Long(_) => true,
        }
    }

    pub fn validate(&self) -> bool {
        for constant in self.entries.iter() {
            if !self.validate_constant(&constant) {
                return false
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Constant::*;

    #[test]
    fn can_add_utf8_constant() {
        let mut cp = ConstantPool::new();
        let expected_utf8 = "java/lang/Object";

        cp.push(Utf8(expected_utf8.to_string()));

        let actual = cp.get_utf8_at(0).unwrap();
        assert_eq!(actual, expected_utf8);
    }

    #[test]
    fn can_add_class_constant() {
        let mut cp = ConstantPool::new();

        cp.push(Utf8("java/lang/Object".to_string())); // idx: 0
        cp.push(Class(0));                             // idx: 1

        assert_eq!(cp.get_class_name(1), "java/lang/Object");
    }

    #[test]
    fn validate_class_constant() {
        let mut cp = ConstantPool::new();

        cp.push(Class(1));

        assert!(!cp.validate());

        cp.push(Utf8("class_name".to_string()));

        assert!(cp.validate());
    }

    #[test]
    fn can_add_name_and_type_constant() {
        let mut cp = ConstantPool::new();

        cp.push(Utf8("name".to_string()));
        cp.push(Utf8("descriptor".to_string()));
        cp.push(NameAndType(0, 1));

        assert!(cp.get_name_and_type(0).is_none());
        assert!(cp.get_name_and_type(1).is_none());
        assert!(cp.get_name_and_type(2).is_some());

        let actual = cp.get_name_and_type(2).unwrap();
        assert!(matches!(*actual, Constant::NameAndType(0, 1)));
    }

    #[test]
    fn validate_name_and_type_constant() {
        let mut cp = ConstantPool::new();

        cp.push(NameAndType(1, 2));
        assert!(!cp.validate());

        cp.push(Utf8("name".to_string()));
        assert!(!cp.validate());

        cp.push(Utf8("descriptor".to_string()));
        assert!(cp.validate());
    }

    fn setup_for_ref_tests(cp: &mut ConstantPool) {
        cp.push(Utf8("SomeClass".to_string()));
        cp.push(Utf8("refname".to_string()));
        cp.push(Utf8("descriptor".to_string()));
        cp.push(NameAndType(1, 2));
        cp.push(Class(0));
    }

    #[test]
    fn can_add_field_ref() {
        let mut cp = ConstantPool::new();

        setup_for_ref_tests(&mut cp);

        cp.push(FieldRef(4, 3));

        assert!(cp.get_field_ref(5).is_some());
    }

    fn test_valid_ref(cp: &mut ConstantPool) {
        // precondition: 0 index has an invalid ref constant
        // pointing to class_index: 1 and name_and_type: 3
        assert!(!cp.validate());

        // class index is valid, but class info is not
        cp.push(Class(2));
        assert!(!cp.validate());

        // class index is valid and class info is valid. But no name and type
        cp.push(Utf8("SomeClass".to_string()));
        assert!(!cp.validate());

        // class index is valid and name and type index is valid. But name and type info is not
        cp.push(NameAndType(4,5));
        assert!(!cp.validate());

        cp.push(Utf8("field1".to_string()));
        cp.push(Utf8("descriptor".to_string()));
        assert!(cp.validate());
    }

    #[test]
    fn validate_field_ref() {
        let mut cp = ConstantPool::new();

        cp.push(FieldRef(1,3));

        test_valid_ref(&mut cp);
    }

    #[test]
    fn can_add_method_ref() {
        let mut cp = ConstantPool::new();

        setup_for_ref_tests(&mut cp);

        cp.push(MethodRef(1, 3));

        assert!(cp.get_method_ref(5).is_some());
    }

    #[test]
    fn validate_method_ref() {
        let mut cp = ConstantPool::new();

        cp.push(MethodRef(1,3));

        test_valid_ref(&mut cp);
    }

    #[test]
    fn can_add_double() {
        let mut cp = ConstantPool::new();
        let double_val = 3.115;

        cp.push(Double(double_val));

        let actual = cp.get_double(0).unwrap();
        assert!(matches!(*&actual, Double(d) if *d == double_val));
    }

    #[test]
    fn can_add_integer() {
        let mut cp = ConstantPool::new();
        let int_val = 42;

        cp.push(Integer(int_val));

        let actual = cp.get_integer(0).unwrap();
        assert!(matches!(*&actual, Integer(i) if *i == int_val));
    }

    #[test]
    fn can_add_float() {
        let mut cp = ConstantPool::new();
        let float_val: f32 = 3.44;

        cp.push(Float(float_val));

        let actual = cp.get_float(0).unwrap();
        assert!(matches!(*&actual, Float(f) if *f == float_val));
    }

    #[test]
    fn can_add_long() {
        let mut cp = ConstantPool::new();
        let long_val: i64 = 13333333337;

        cp.push(Long(long_val));

        let actual = cp.get_long(0).unwrap();
        assert!(matches!(*&actual, Long(l) if *l == long_val));
    }
}
