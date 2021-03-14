
// Declare custom app errors


apptools::declare_app_errors!(ErrList, ErrListLookupTable,
    UserHasNoProfile, "user has no profile",
    RequestLimitExceeded, "user exceeded the number of requests per time unit",
    DummyError, "a dummy error"
);
