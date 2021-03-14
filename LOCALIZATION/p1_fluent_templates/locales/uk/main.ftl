hello-world = Вітання від Fluent!
greeting = Вітаю, { $name }!
Yuri = Юрію
shared-photos =
    {$userName} {$userGender ->
        [male] додав
        [female] додала
       *[other] додали
    } {$photoCount} {$photoCount ->
        [one]  нову фотографію
        [few] нові фотографії
       *[other] нових фотографій
    } до {$userGender ->
        [male] свого стріму
        [female] свого стріму
       *[other] свого стріму
    }.
