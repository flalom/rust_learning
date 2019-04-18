// In this script I will copy and study the rust book examples
// For example this structure or struc it is a nice inclusion to make sort of initiator.
//
//A struct, or structure, is a custom data type that lets you name and package together multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.
//
//Using the Field Init Shorthand When Variables and Fields Have the Same Name
//Because the parameter names and the struct field names are exactly the same in Listing 5-4, we can use the field init shorthand syntax to rewrite build_user so that it behaves exactly the same but doesn’t have the repetition of email and username, as shown in Listing 5-5.
//Of course all this function they should be within the main function.

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
