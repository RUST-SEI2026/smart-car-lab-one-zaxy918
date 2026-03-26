use executor::{Executor, Pose};
mod move_tests {
    use super::*;

    #[test]
    fn ret_1_given_cmd_m_facing_e() {
        let origin_pose = Pose::new(0, 0, 'E');
        let mut ececutor = Executor::with_pose(origin_pose);
        ececutor.execute("M");
        let expected_pose = Pose::new(1, 0, 'E');
        assert_eq!(ececutor.query(), expected_pose);
    }

    #[test]
    fn ret_1_given_cmd_m_facing_s() {
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'S'));
        executor.execute("M");
        assert_eq!(executor.query(), Pose::new(0, -1, 'S'));
    }

    #[test]
    fn ret_1_given_cmd_m_facing_w() {
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'W'));
        executor.execute("M");
        assert_eq!(executor.query(), Pose::new(-1, 0, 'W'));
    }

    #[test]
    fn ret_1_given_cmd_m_facing_n() {
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
        executor.execute("M");
        assert_eq!(executor.query(), Pose::new(0, 1, 'N'));
    }
}

mod change_pose_tests {

    use super::*;

    mod turn_left_tests {
        use super::*;
        #[test]
        fn ret_1_given_cmd_l_facing_e() {
            let mut executor = Executor::with_pose(Pose::new(0, 0, 'E'));
            executor.execute("L");
            assert_eq!(executor.query(), Pose::new(0, 0, 'N'));
        }

        #[test]
        fn ret_1_given_cmd_l_facing_w() {
            let mut executor = Executor::with_pose(Pose::new(0, 0, 'W'));
            executor.execute("L");
            assert_eq!(executor.query(), Pose::new(0, 0, 'S'));
        }

        #[test]
        fn ret_1_given_cmd_l_facing_n() {
            let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
            executor.execute("L");
            assert_eq!(executor.query(), Pose::new(0, 0, 'W'));
        }

        #[test]
        fn ret_1_given_cmd_l_facing_s() {
            let mut executor = Executor::with_pose(Pose::new(0, 0, 'S'));
            executor.execute("L");
            assert_eq!(executor.query(), Pose::new(0, 0, 'E'));
        }
    }

    mod turn_right_tests {
        use super::*;

        #[test]
        fn ret_1_given_cmd_r_facing_e() {
            let mut executor = Executor::with_pose(Pose::new(0, 0, 'E'));
            executor.execute("R");
            assert_eq!(executor.query(), Pose::new(0, 0, 'S'));
        }

        #[test]
        fn ret_1_given_cmd_r_facing_w() {
            let mut executor = Executor::with_pose(Pose::new(0, 0, 'W'));
            executor.execute("R");
            assert_eq!(executor.query(), Pose::new(0, 0, 'N'));
        }

        #[test]
        fn ret_1_given_cmd_r_facing_n() {
            let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
            executor.execute("R");
            assert_eq!(executor.query(), Pose::new(0, 0, 'E'));
        }

        #[test]
        fn ret_1_given_cmd_r_facing_s() {
            let mut executor = Executor::with_pose(Pose::new(0, 0, 'S'));
            executor.execute("R");
            assert_eq!(executor.query(), Pose::new(0, 0, 'W'));
        }
    }
}

mod batch_cmd_tests {

    use super::*;

    #[test]
    fn ret_1_given_cmd_longer_than_1() {
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
        executor.execute("MLMRMMMLLMRR");
        assert_eq!(executor.query(), Pose::new(-1, 3, 'N'));
    }
}

mod query_tests {

    use super::*;

    #[test]
    fn ret_1_call_query() {
        let executor = Executor::with_pose(Pose::new(2, 3, 'W'));
        assert_eq!(executor.query(), Pose::new(2, 3, 'W'));
    }
}
