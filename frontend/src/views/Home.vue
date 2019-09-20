<template>
  <div class="home">
    <h2>Anonymous Issue Submit</h2>
    <b>This small app allows you to anonymously submit issues to the IfI quality management issue system</b>
    <form class="issue-form" v-on:submit="submit">
      <input v-model="title" class="issue-title-input" autofocus min="1" required placeholder="Title">
      <vue-simplemde class="issue-editor" v-model="description" ref="markdownEditor" />
      <template v-if="!issueLink">
        Confidential? <input v-model="confidential" type="checkbox">
        <input type="submit" value="Submit issue" :disabled="submitDisabled">
      </template>
    </form>
    <span>{{message}}</span><a :href="issueLink" v-if="issueLink" >{{issueLink}}</a>
  </div>
</template>

<script lang="ts">
import Vue from 'vue';
import VueSimplemde from 'vue-simplemde'; // @ is an alias to /src
import axios from 'axios';

function getInstanceBaseUrl(): string {
  if (process.env.NODE_ENV === 'production' || process.env.NODE_ENV === 'test') {
    return `${window.location.protocol}//${window.location.host}${window.location.pathname}`.replace(/\/+$/, '');
  } else {
    return 'http://localhost:8000/';
  }
}

const ax = axios.create({
  baseURL: getInstanceBaseUrl(),
});

export default Vue.extend({
  name: 'home',
  components: {
    VueSimplemde,
  },
  data: () => {
    return {
      description: '',
      confidential: false,
      title: '',
      message: '',
      issueLink: '',
      submitDisabled: false,
    };
  },
  methods: {
    submit() {
      if (this.description.trim().length === 0) {
        this.message = 'You need to enter a description!';
        this.issueLink = '';
        return;
      }

      this.submitDisabled = true;
      const issue = {
        title: '[Anonymous] ' + this.title,
        description: this.description,
        confidential: this.confidential,
      };
      ax.post('/api/issue', issue).then((response) => {
        this.message = 'Created new anonymous issue at ';
        this.issueLink = response.data.web_url;
      }).catch((err) => {
        this.message = `Failed to create issue: ${err}`;
        this.issueLink = '';
      }).finally(() => {
        this.submitDisabled = false;
      });
    },
  },
});
</script>

<style>
  @import '~simplemde/dist/simplemde.min.css';

  .issue-form {
    margin-top: 20px;
  }

  .issue-title-input {
    width: 890px;
    max-width: 100%;
    font-size: large;
  }

  .issue-editor {
    width: 900px;
    max-width: 100%;
    margin-left: auto;
    margin-right: auto;
    text-align: left;
  }
</style>
