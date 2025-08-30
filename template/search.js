function initSearch() {
    const searchBox = document.getElementById('search');
    const functions = document.querySelectorAll('.function');
    const navItems = document.querySelectorAll('.nav-item');
    
    document.querySelectorAll('.nav-title').forEach(title => {
        title.addEventListener('click', function() {
            const section = this.parentElement;
            section.classList.toggle('collapsed');
        });
    });
    
    searchBox.addEventListener('input', function() {
        const query = this.value.toLowerCase();
        
        functions.forEach(func => {
            const name = func.dataset.name.toLowerCase();
            const desc = func.dataset.description.toLowerCase();
            
            if (name.includes(query) || desc.includes(query)) {
                func.style.display = 'block';
            } else {
                func.style.display = 'none';
            }
        });
        
        navItems.forEach(item => {
            const link = item.querySelector('.nav-link');
            const name = link.textContent.toLowerCase();
            
            if (name.includes(query)) {
                item.style.display = 'block';
                const section = item.closest('.nav-section');
                section.classList.remove('collapsed');
            } else {
                item.style.display = 'none';
            }
        });
    });
    
    const observer = new IntersectionObserver(entries => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                const id = entry.target.id;
                document.querySelectorAll('.nav-link').forEach(link => {
                    link.classList.remove('active');
                    if (link.getAttribute('href') === '#' + id) {
                        link.classList.add('active');
                        const section = link.closest('.nav-section');
                        section.classList.remove('collapsed');
                    }
                });
            }
        });
    }, { threshold: 0.5 });
    
    functions.forEach(func => observer.observe(func));
}

document.addEventListener('DOMContentLoaded', initSearch);