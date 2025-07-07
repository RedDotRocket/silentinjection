# Silent Injection: Hugging Face Security Research

A minimalist landing page presenting research findings on widespread security risks in Hugging Face model usage across AI applications.

## 🚀 Quick Start

This site is designed to be deployed to GitHub Pages with minimal configuration. The landing page presents research by Luke Hinds and Fabian Kammel on security misconfigurations in Hugging Face model implementations.

## 📁 Project Structure

```
├── index.html              # Main landing page
├── styles.css              # Minimalist styling and responsive design
├── .github/
│   └── workflows/
│       └── pages.yml       # GitHub Actions workflow for deployment
└── README.md               # This file
```

## 🌐 Deploying to GitHub Pages

### Option 1: Automatic Deployment (Recommended)

1. **Enable GitHub Pages in your repository:**
   - Go to your repository settings
   - Scroll down to "Pages" section
   - Under "Source", select "GitHub Actions"

2. **Push your code to the main branch:**
   ```bash
   git add .
   git commit -m "Add landing page for Hugging Face security research"
   git push origin main
   ```

3. **The GitHub Action will automatically deploy your site:**
   - The workflow in `.github/workflows/pages.yml` will trigger on push
   - Your site will be available at `https://[username].github.io/[repository-name]`
   - Check the "Actions" tab to monitor deployment progress

### Option 2: Manual GitHub Pages Setup

If you prefer not to use GitHub Actions:

1. **Go to Repository Settings:**
   - Navigate to your repository on GitHub
   - Click on "Settings" tab

2. **Configure Pages:**
   - Scroll to "Pages" section
   - Under "Source", select "Deploy from a branch"
   - Choose "main" branch and "/ (root)" folder
   - Click "Save"

3. **Access your site:**
   - Your site will be available at `https://[username].github.io/[repository-name]`
   - It may take a few minutes for the site to become available

## 🛠 Local Development

To preview the site locally:

1. **Clone the repository:**
   ```bash
   git clone [your-repository-url]
   cd [repository-name]
   ```

2. **Serve the files locally:**
   
   **Option A: Using Python (if installed):**
   ```bash
   # Python 3
   python -m http.server 8000
   
   # Python 2
   python -m SimpleHTTPServer 8000
   ```
   
   **Option B: Using Node.js (if installed):**
   ```bash
   npx serve .
   ```
   
   **Option C: Using any other static file server**

3. **Open your browser:**
   - Navigate to `http://localhost:8000`
   - The site should load with full styling and responsiveness

## 🎨 Customization

### Adding Images

The landing page includes placeholder sections for visual diagrams. To add images:

1. **Create an `images/` directory:**
   ```bash
   mkdir images
   ```

2. **Add your images to the directory**

3. **Replace image placeholders in `index.html`:**
   ```html
   <!-- Replace this: -->
   <div class="image-placeholder">
       <div class="placeholder-content">
           <span>📊 Visual: Attack Flow Diagram</span>
           <p>Diagram showing how compromised upstream models propagate to downstream applications</p>
       </div>
   </div>
   
   <!-- With this: -->
   <div class="image-container">
       <img src="images/attack-flow-diagram.png" alt="Attack Flow Diagram showing how compromised upstream models propagate to downstream applications">
   </div>
   ```

4. **Add corresponding CSS for image styling in `styles.css`:**
   ```css
   .image-container {
       margin: 3rem 0;
       text-align: center;
   }
   
   .image-container img {
       max-width: 100%;
       height: auto;
       border-radius: 8px;
       box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
   }
   ```

### Styling Customization

The CSS is organized into clear sections:

- **Typography:** Font choices and text styling
- **Layout:** Grid systems and spacing
- **Components:** Cards, code blocks, and interactive elements
- **Responsive Design:** Mobile-first approach with breakpoints
- **Accessibility:** Focus states and reduced motion support

Key CSS custom properties can be modified at the top of `styles.css` for easy theming.

### Content Updates

To update the research content:

1. **Edit `index.html`** for content changes
2. **Modify `styles.css`** for styling adjustments
3. **Test locally** before pushing changes
4. **Push to main branch** for automatic deployment (if using GitHub Actions)

## 🔧 Technical Features

- **Responsive Design:** Mobile-first approach with clean breakpoints
- **Semantic HTML:** Proper markup for accessibility and SEO
- **Performance Optimized:** Minimal CSS and efficient loading
- **Code Highlighting:** Styled code blocks with syntax awareness
- **Print Friendly:** Optimized styles for printing
- **Accessibility:** WCAG compliant with proper focus management

## 📊 Browser Support

The site supports all modern browsers:

- Chrome 60+
- Firefox 60+
- Safari 12+
- Edge 79+

## 🔍 SEO Optimization

The site includes:

- Semantic HTML structure
- Meta descriptions and keywords
- Open Graph tags (can be added for social sharing)
- Proper heading hierarchy
- Alt text for images (when added)

## 🚨 Security Research Context

This landing page presents research findings on:

- **Supply Chain Attacks** in AI/ML pipelines
- **Model Poisoning** through upstream compromise
- **Trigger-Based Backdoors** in language models
- **Detection and Remediation** strategies

The research emphasizes the importance of pinning Hugging Face model revisions to prevent silent security degradation.

## 📄 License

This project contains security research findings. Please respect the research attribution when sharing or modifying the content.

## 🐛 Issues and Contributions

If you encounter issues with the deployment or have suggestions for improvements:

1. Check that GitHub Pages is properly configured in your repository settings
2. Verify that the GitHub Actions workflow has the necessary permissions
3. Ensure your repository is public (required for free GitHub Pages)

For technical issues with the landing page itself, please check the browser console for any JavaScript errors and verify all file paths are correct.