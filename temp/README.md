reference.html will be used as the style reference for the frontend and dictates how the site should look and feel for PC and mobile touch devices.


The Marquee will contain a tech meditation of they day

Website workflow
- Website opens with only showing the CardStack components which are CardStack.astro components.
- Each CardStack represents a subfolder in ./content/ folder and one CardStack is for blog posts, one for github and one for projects. It would be ideal if these CardStack components are created based on the subfolders in ./content 
- When a CardStack is clicked, Card components are placed on the website after the CardStack components in the grid, that is to say that if project CardStack is clicked then Card components representing ./content/project/project1.md and ./content/project/project2.md are placed on the webpage. If the project CardStack is clicked again the Card components are removed from the webpage. There should be an animation for Card componentss being "dealt" from the CardStack and returned to the CardStack. These should be placed in CardInteractions.astro
- When a Card component is clicked or tapped, the contents of the related converted markdown is rendered in the modal popup.
- The markdown should have an absolute path to an image that will be the face image of the Card.