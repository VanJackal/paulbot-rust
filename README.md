# PaulBot Rust
A rewrite of [PaulBot](https://github.com/VanJackal/PaulBot) in Rust with improved UX using newer features of the Discord API.

# Features
## Commands
`/meow`
Replies with a meow message.

`/pet [cat]`
Reply with an image of a cat.

`/petpetpet [cat] [amount]`
Reply with `amount` random cat images, defaults to 3 images.

`/feed [cat]`
Feeds a cat and increments the cat's feed stat.

`/info [user|cat] [cat]`
Shows a bunch of stats for a user, cat, specified cat for user, or generic stats for the bot.

## Adding Cats & Pictures
Cat pictures can be added via a right-click context menu on chat images. It should provide a menu for adding details 
for the image.


# TODO

## Commands
- [x] Implement `/meow`
    - [x] Reply with a meow message
    - [x] store a set of cat meow messages (probably JSON text file, doesn't need a DB)
    - [ ] stats tracking
- [ ] Implement `/pet [cat]`
    - [ ] Reply with an image (url) of a cat
    - [ ] cat images should have a brief title that can be used for link formatting
    - [ ] stats tracking
- [ ] Implement `/petpetpet [cat] [amount]`
    - [ ] Reply with `amount` random cat images
    - [ ] stats tracking
- [ ] Implement `/feed [cat]`
    - [ ] Feed a cat
    - [ ] Increment the cat's feed stat
- [ ] Implement `/info [user|cat] [cat]`
    - [ ] Show stats for a user
    - [ ] Show stats for a cat
    - [ ] Show stats for a specified cat owned by a user
    - [ ] Show generic stats for the bot

## Adding Cats & Pictures
- [ ] Add cat pictures through a right-click context menu on chat images
    - [ ] Provide a menu for adding image details
    - [ ] Save the image metadata for later use
- [ ] Downloading cat picture from discord

## Data 
- [ ] SQLite DB Schema definition
- [ ] S3 Bucket for cat pictures
  - [ ] upload
  - [ ] can I add metadata to S3 items?
  - [ ] images should be hashed to avoid duplicates