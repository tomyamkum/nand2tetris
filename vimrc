set title
set fenc=utf-8
set nobackup
set noswapfile
set autoread
set hidden
set showcmd

set number
:syntax on
set virtualedit=onemore
set smartindent
set showmatch
set matchtime=1
set laststatus=2
highlight Statusline term=NONE cterm=NONE ctermfg=white ctermbg=black
set wildmode=list:longest
nnoremap j gj
nnoremap k gk

set tabstop=2
set shiftwidth=2
set expandtab

set ignorecase
set smartcase
set incsearch
set wrapscan
set hlsearch
nmap <Esc><Esc> :nohlsearch<CR><Esc>
set nrformats=

set clipboard+=unnamed
set backspace=indent,eol,start

inoremap <silent> <C-j> <Esc>
