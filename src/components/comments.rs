use leptos::prelude::*;

// Represents a single discussion comment, including any nested replies.
#[derive(Clone)]
pub struct Comment {
    pub id: String,
    pub author: String,
    pub avatar: String,
    pub content: String,
    pub time_ago: String,
    pub upvotes: i32,
    pub replies: Vec<Comment>, // Child comments rendered as a threaded conversation.
}

// Renders a single comment node, including its nested reply tree.
#[component]
pub fn CommentItem(comment: Comment) -> impl IntoView {
    view! {
        <div class="comment-thread">
            <div class="comment-main">
                // Left column containing the avatar and optional thread connector.
                <div class="comment-left">
                    <div class="comment-avatar">{comment.avatar.clone()}</div>
                    // Render the vertical connector only when the comment has replies.
                    {if !comment.replies.is_empty() {
                        view! { <div class="thread-line"></div> }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }}
                </div>

                // Main content area for comment metadata, body, and actions.
                <div class="comment-body">
                    <div class="comment-header">
                        <span class="comment-author">{comment.author.clone()}</span>
                        <span class="comment-time">{comment.time_ago.clone()}</span>
                    </div>
                    <div class="comment-content">
                        {comment.content.clone()}
                    </div>
                    <div class="comment-actions">
                        <button class="action-btn">"↑ " {comment.upvotes} " Upvotes"</button>
                        <button class="action-btn reply-btn">"Reply"</button>
                    </div>
                </div>
            </div>

            // Recursively render all nested replies for the current comment.
            <div class="comment-replies">
                {comment.replies.into_iter().map(|reply| {
                    view! {
                        <CommentItem comment=reply />
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

// Top-level discussion container responsible for rendering the comment section.
#[component]
pub fn IssueDiscussion() -> impl IntoView {
    // Mock discussion data used to simulate a threaded conversation.
    let comments = vec![
        Comment {
            id: "1".into(),
            author: "Jozef Mak".into(),
            avatar: "JM".into(),
            content: "Myslím, že by sme tu mali použiť PostgreSQL namiesto MySQL. Bude to lepšie škálovať.".into(),
            time_ago: "2 hours ago".into(),
            upvotes: 5,
            replies: vec![
                Comment {
                    id: "2".into(),
                    author: "Peter Hraško".into(),
                    avatar: "PH".into(),
                    content: "Súhlasím, najmä kvôli podpore JSONB stĺpcov, ktoré budeme potrebovať pre tie dynamické formuláre.".into(),
                    time_ago: "1 hour ago".into(),
                    upvotes: 3,
                    replies: vec![
                        Comment {
                            id: "3".into(),
                            author: "Jozef Mak".into(),
                            avatar: "JM".into(),
                            content: "Presne! Založím na to samostatný task.".into(),
                            time_ago: "15 mins ago".into(),
                            upvotes: 1,
                            replies: vec![],
                        }
                    ],
                }
            ],
        },
        Comment {
            id: "4".into(),
            author: "Jana Nováková".into(),
            avatar: "JN".into(),
            content: "Nezabudnite prosím pridať aj indexy na cudzie kľúče, minule nám to pri testoch padalo na výkone.".into(),
            time_ago: "30 mins ago".into(),
            upvotes: 8,
            replies: vec![],
        }
    ];

    view! {
        <section class="discussion-section">
            <h3 class="discussion-title">"Discussion"</h3>
            
            // Input area for submitting a new comment.
            <div class="new-comment-box">
                <div class="comment-avatar">"Me"</div>
                <div class="new-comment-input-wrap">
                    <textarea placeholder="Add a comment... (Markdown supported)"></textarea>
                    <div class="new-comment-actions">
                        <button class="primary-button" style="height: 32px; font-size: 0.8rem;">"Comment"</button>
                    </div>
                </div>
            </div>

            // Container for rendering all top-level discussion threads.
            <div class="threads-container">
                {comments.into_iter().map(|c| view! { <CommentItem comment=c /> }).collect_view()}
            </div>
        </section>
    }
}