hello-world = Приветствие от Fluent!
greeting = Приветствую, { $name }!
Yuri = Юрий
shared-photos =
    {$userName} {$userGender ->
        [male] добавил
        [female] добавила
       *[other] добавили
    } {$photoCount} {$photoCount ->
        [one]  новую фотографию
        [few] новых фотографии
       *[other] новых фотографий
    } к {$userGender ->
        [male] своему стриму
        [female] своему стриму
       *[other] своему стриму
    }.
