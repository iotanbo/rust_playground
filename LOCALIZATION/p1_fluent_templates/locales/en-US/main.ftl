hello-world = Greetings from Fluent!
greeting = Hello { $name }!
Yuri = Yuri

shared-photos =
    {$userName} added {$photoCount} {$photoCount ->
        [one]  new photo
       *[other] new photos
    } to {$userGender ->
        [male] his stream
        [female] her stream
       *[other] their stream
    }.
