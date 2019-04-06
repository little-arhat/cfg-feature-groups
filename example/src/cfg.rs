// macro_rules! feature_group {
//     ()
// }

// feature group lol
#[cfg(not(any(feature = "opt1", feature = "opt2", feature = "opt3")))]
compile_error!("One of the opt1,opt2,opt3 should be provided for feature group `lol`");

#[cfg(all(feature = "opt1", feature = "opt2"))]
compile_error!("Only one of the opt1,opt2,opt3 should be provided for feature group `lol`");

#[cfg(all(feature = "opt2", feature = "opt3"))]
compile_error!("Only one of the opt1,opt2,opt3 should be provided for feature group `lol`");

#[cfg(all(feature = "opt1", feature = "opt3"))]
compile_error!("Only one of the opt1,opt2,opt3 should be provided for feature group `lol`");
