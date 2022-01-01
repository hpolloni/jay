
#[derive(Debug)]
pub enum Constant {
    Class(usize),
    Utf8(String),
    NameAndType(usize, usize),
    FieldRef(usize, usize),
    MethodRef(usize, usize)
}

pub struct ConstantPool {
    entries: Vec<Constant>
}

impl ConstantPool {
    pub fn new() -> Self {
        let entries = Vec::new();
        Self { entries }
    }

    pub fn add_utf8(&mut self, str: String) {
        self.entries.push(Constant::Utf8(str));
    }

    pub fn get_utf8_at(&self, index: usize) -> Option<&Constant> {
        self.entries.get(index).filter(|c| matches!(c, Constant::Utf8(_)))
    }

    pub fn add_class(&mut self, name_index: usize) {
        self.entries.push(Constant::Class(name_index));
    }

    pub fn get_class_at(&self, index: usize) -> Option<&Constant> {
        self.entries.get(index).filter(|c| matches!(c, Constant::Class(_)))
    }

    pub fn add_name_and_type(&mut self, name_index: usize, descriptor_index: usize) {
        self.entries.push(Constant::NameAndType(name_index, descriptor_index));
    }

    pub fn get_name_and_type(&self, index: usize) -> Option<&Constant> {
        self.entries.get(index).filter(|c| matches!(c, Constant::NameAndType(_,_)))
    }

    pub fn add_field_ref(&mut self, class_index: usize, name_and_type_index: usize) {
        self.entries.push(Constant::FieldRef(class_index, name_and_type_index));
    }

    pub fn get_field_ref(&mut self, index: usize) -> Option<&Constant> {
        self.entries.get(index).filter(|c| matches!(c, Constant::FieldRef(_,_)))
    }

    pub fn add_method_ref(&mut self, class_index: usize, name_and_type_index: usize) {
        self.entries.push(Constant::MethodRef(class_index, name_and_type_index));
    }

    pub fn get_method_ref(&mut self, index: usize) -> Option<&Constant> {
        self.entries.get(index).filter(|c| matches!(c, Constant::MethodRef(_,_)))
    }

    fn validate_constant(&self, constant: &Constant) -> bool {
        match constant {
            Constant::Class(n) => self.get_utf8_at(*n).is_some(),
            Constant::Utf8(_) => true,
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
            }
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

    #[test]
    fn can_add_utf8_constant() {
        let mut cp = ConstantPool::new();
        let expected_utf8 = "java/lang/Object";

        cp.add_utf8(expected_utf8.to_string());

        assert!(cp.get_utf8_at(0).is_some());
        let actual = cp.get_utf8_at(0).unwrap();
        assert!(matches!(*&actual, Constant::Utf8(str) if str == expected_utf8));
    }

    #[test]
    fn can_add_class_constant() {
        let mut cp = ConstantPool::new();

        cp.add_utf8("java/lang/Object".to_string());
        cp.add_class(0);

        assert!(cp.get_class_at(0).is_none());
        assert!(cp.get_class_at(1).is_some());

        let actual = cp.get_class_at(1).unwrap();
        assert!(matches!(*actual, Constant::Class(0)));
    }

    #[test]
    fn validate_class_constant() {
        let mut cp = ConstantPool::new();

        cp.add_class(1);

        assert!(!cp.validate());

        cp.add_utf8("class_name".to_string());

        assert!(cp.validate());
    }

    #[test]
    fn can_add_name_and_type_constant() {
        let mut cp = ConstantPool::new();

        cp.add_utf8("name".to_string());
        cp.add_utf8("descriptor".to_string());
        cp.add_name_and_type(0, 1);

        assert!(cp.get_name_and_type(0).is_none());
        assert!(cp.get_name_and_type(1).is_none());
        assert!(cp.get_name_and_type(2).is_some());

        let actual = cp.get_name_and_type(2).unwrap();
        assert!(matches!(*actual, Constant::NameAndType(0, 1)));
    }

    #[test]
    fn validate_name_and_type_constant() {
        let mut cp = ConstantPool::new();

        cp.add_name_and_type(1, 2);
        assert!(!cp.validate());

        cp.add_utf8("name".to_string());
        assert!(!cp.validate());

        cp.add_utf8("descriptor".to_string());
        assert!(cp.validate());
    }

    fn setup_for_ref_tests(cp: &mut ConstantPool) {
        cp.add_utf8("SomeClass".to_string());
        cp.add_utf8("refname".to_string());
        cp.add_utf8("descriptor".to_string());
        cp.add_name_and_type(1, 2);
        cp.add_class(0);
    }

    #[test]
    fn can_add_field_ref() {
        let mut cp = ConstantPool::new();

        setup_for_ref_tests(&mut cp);

        cp.add_field_ref(4, 3);

        assert!(cp.get_field_ref(5).is_some());
    }

    fn test_valid_ref(cp: &mut ConstantPool) {
        // precondition: 0 index has an invalid ref constant
        // pointing to class_index: 1 and name_and_type: 3
        assert!(!cp.validate());

        // class index is valid, but class info is not
        cp.add_class(2);
        assert!(!cp.validate());

        // class index is valid and class info is valid. But no name and type
        cp.add_utf8("SomeClass".to_string());
        assert!(!cp.validate());

        // class index is valid and name and type index is valid. But name and type info is not
        cp.add_name_and_type(4,5);
        assert!(!cp.validate());

        cp.add_utf8("field1".to_string());
        cp.add_utf8("descriptor".to_string());
        assert!(cp.validate());
    }

    #[test]
    fn validate_field_ref() {
        let mut cp = ConstantPool::new();

        cp.add_field_ref(1,3);

        test_valid_ref(&mut cp);
    }

    #[test]
    fn can_add_method_ref() {
        let mut cp = ConstantPool::new();

        setup_for_ref_tests(&mut cp);

        cp.add_method_ref(1, 3);

        assert!(cp.get_method_ref(5).is_some());
    }

    #[test]
    fn validate_method_ref() {
        let mut cp = ConstantPool::new();

        cp.add_method_ref(1,3);

        test_valid_ref(&mut cp);
    }
}
