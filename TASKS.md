# Frontend

- The "please login" states of /profile is bland.
- There is no live section

# Backend

- Make upvotes/downvotes work
- Add replying to replies
- Every content, page will have a dedicated id (already does) and that id will be used (in backend) to hash the senders ids/names etc to some random value. Then in page contents, that hash will be rendered so you can see if someone commented twice, or if they replied to themselves (shows same hash). But for a different content, that same user will have a different hash making every hash depend on the content, like posts!
