# Frontend

- Auth isnt designed, or looks same as everything else, it is completely out of touch.
- The "please login" states of each page seems super empty and bland
- Some of the themes are completely off from what you have given me, for example midnight is blue no instead of green but its not a problem as long as their mid/dark modes were different enough but they arent. This is the same scenario in Pinewood as well, which you didnt ask me before asking but it looks super good (except mid and dark look same).
- After hovering over the profile name, it does open the options correctly but moving the mouse towards there instantly closes it making it impossible to get to settings etc.
- While hovering over the breadcrumbs, make it obvious that they are clickable by adding a background etc maybe
- Upvotes/downvotes arent clickable (backend issue)
- There is no way to upvote/downvote (visually) in posts.

# Backend

- Make upvotes/downvotes work
- Add replying to replies
- Every content, page will have a dedicated id (already does) and that id will be used (in backend) to hash the senders ids/names etc to some random value. Then in page contents, that hash will be rendered so you can see if someone commented twice, or if they replied to themselves (shows same hash). But for a different content, that same user will have a different hash making every hash depend on the content, like posts!
