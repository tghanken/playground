import '../input.css';
import Alpine from 'alpinejs';
import morph from '@alpinejs/morph';
import ajax from '@imacrayon/alpine-ajax';

window.Alpine = Alpine;
Alpine.plugin(morph);
Alpine.plugin(ajax);
