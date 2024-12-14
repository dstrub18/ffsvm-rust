macro_rules! test_model {
    ($name:ident, $file:expr, $prob:expr, $libsvm:expr, $libsvm_prob:expr) => {
        #[test]
        fn $name() -> Result<(), Error> {
            let model = include_str!(concat!("data_dense/", $file));
            let svm = DenseSVM::try_from(model)?;

            let mut problem_0 = FeatureVector::from(&svm);
            let features_0 = problem_0.features();
            features_0.clone_from_slice(&[0.000_1, 0.000_1, 0.000_1, 0.000_1, 0.000_1, 0.000_1, 0.000_1, 0.000_1]);

            let mut problem_7 = FeatureVector::from(&svm);
            let features_7 = problem_7.features();
            features_7.clone_from_slice(&[1.287_784_9, 0.986_031_7, 1.486_247_2, 1.128_083, 0.891_030_55, 1.164_363_4, 0.928_599_1, 1.140_762_9]);

            svm.predict_value(&mut problem_0)?;
            svm.predict_value(&mut problem_7)?;

            assert_eq!(problem_0.label(), Label::Class($libsvm[0]), "predict_value(problem_0)");
            assert_eq!(problem_7.label(), Label::Class($libsvm[1]), "predict_value(problem_7)");

            if $prob {
                svm.predict_probability(&mut problem_0)?;
                svm.predict_probability(&mut problem_7)?;

                assert_eq!(problem_0.label(), Label::Class($libsvm_prob[0]), "predict_probability(problem_0)");
                assert_eq!(problem_7.label(), Label::Class($libsvm_prob[1]), "predict_probability(problem_7)");
            }

            Ok(())
        }
    };
}

#[cfg(test)]
mod svm_dense_class {
    use ffsvm::{DenseSVM, Error, FeatureVector, Label, Predict};
    use std::convert::TryFrom;

    // CSVM
    test_model!(m_csvm_linear_prob, "m_csvm_linear_prob.libsvm", true, [0, 7], [0, 7]);
    test_model!(m_csvm_poly_prob, "m_csvm_poly_prob.libsvm", true, [0, 7], [5, 7]); // apparently `libSVM` gets this wrong
    test_model!(m_csvm_rbf_prob, "m_csvm_rbf_prob.libsvm", true, [0, 7], [2, 7]); // apparently `libSVM` gets this wrong
    test_model!(m_csvm_sigmoid_prob, "m_csvm_sigmoid_prob.libsvm", true, [0, 5], [0, 7]); // apparently `libSVM` gets this wrong

    // Temporarily disabled as they trigger ICE in Rust Nightly
    test_model!(m_csvm_linear, "m_csvm_linear.libsvm", false, [0, 7], [0, 0]);
    test_model!(m_csvm_poly, "m_csvm_poly.libsvm", false, [0, 7], [0, 0]);
    test_model!(m_csvm_rbf, "m_csvm_rbf.libsvm", false, [0, 7], [0, 0]);
    test_model!(m_csvm_sigmoid, "m_csvm_sigmoid.libsvm", false, [0, 5], [0, 0]);

    // NUSVM
    test_model!(m_nusvm_linear_prob, "m_nusvm_linear_prob.libsvm", true, [0, 7], [0, 7]);
    test_model!(m_nusvm_poly_prob, "m_nusvm_poly_prob.libsvm", true, [0, 7], [0, 7]);
    test_model!(m_nusvm_rbf_prob, "m_nusvm_rbf_prob.libsvm", true, [0, 7], [0, 7]);
    test_model!(m_nusvm_sigmoid_prob, "m_nusvm_sigmoid_prob.libsvm", true, [0, 7], [0, 7]);

    // Temporarily disabled as they trigger ICE in Rust Nightly
    test_model!(m_nusvm_linear, "m_nusvm_linear.libsvm", false, [0, 7], [0, 0]);
    test_model!(m_nusvm_poly, "m_nusvm_poly.libsvm", false, [0, 7], [0, 0]);
    test_model!(m_nusvm_rbf, "m_nusvm_rbf.libsvm", false, [0, 7], [0, 0]);
    test_model!(m_nusvm_sigmoid, "m_nusvm_sigmoid.libsvm", false, [0, 7], [0, 0]);
}
