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
}
