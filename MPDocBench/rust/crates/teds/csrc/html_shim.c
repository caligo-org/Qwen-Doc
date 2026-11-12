/*
 * Thin shim over libxml2's HTML parser that sets the parser up the way
 * lxml 5.3.0's `lxml.html.HTMLParser(remove_comments=True)` does, and
 * exposes the few node fields the Rust side reads.
 */
#include <stdlib.h>
#include <string.h>
#include <libxml/parser.h>
#include <libxml/HTMLparser.h>
#include <libxml/tree.h>
#include <libxml/xmlerror.h>
#include <libxml/xmlversion.h>

static void teds_silent_structured(void *ctx, xmlErrorPtr err) {
    (void)ctx;
    (void)err;
}

static void teds_silent_generic(void *ctx, const char *msg, ...) {
    (void)ctx;
    (void)msg;
}

void teds_init(void) {
    xmlInitParser();
}

int teds_libxml_version(void) {
    return xmlParserVersion ? atoi(xmlParserVersion) : LIBXML_VERSION;
}

/* lxml: HTMLParser defaults are RECOVER | NONET | COMPACT. */
#define TEDS_HTML_OPTIONS (HTML_PARSE_RECOVER | HTML_PARSE_NONET | HTML_PARSE_COMPACT)

xmlDocPtr teds_parse(const char *buf, int len, const char *encoding) {
    htmlParserCtxtPtr ctxt;
    xmlSAXHandlerPtr sax;
    xmlDocPtr doc;

    xmlSetGenericErrorFunc(NULL, teds_silent_generic);
    xmlSetStructuredErrorFunc(NULL, teds_silent_structured);

    /* lxml: _BaseParser._newParserCtxt */
    ctxt = htmlCreateMemoryParserCtxt("dummy", 5);
    if (ctxt == NULL)
        return NULL;
    sax = ctxt->sax;
    /* lxml: _registerHtmlErrorHandler */
    if (sax != NULL && sax->initialized && sax->initialized != XML_SAX2_MAGIC) {
        if ((void *)sax == (void *)&htmlDefaultSAXHandler) {
            sax = (xmlSAXHandlerPtr)xmlMalloc(sizeof(xmlSAXHandler));
            if (sax == NULL) {
                htmlFreeParserCtxt(ctxt);
                return NULL;
            }
            memcpy(sax, &htmlDefaultSAXHandler, sizeof(htmlDefaultSAXHandler));
            ctxt->sax = sax;
        }
        sax->initialized = XML_SAX2_MAGIC;
        sax->serror = teds_silent_structured;
        sax->startElementNs = NULL;
        sax->endElementNs = NULL;
        sax->_private = NULL;
    }
    /* lxml: _configureSaxContext (remove_comments=True, strip_cdata=True) */
    sax->comment = NULL;
    sax->cdataBlock = NULL;

    doc = htmlCtxtReadMemory(ctxt, buf, len, NULL, encoding, TEDS_HTML_OPTIONS);
    if (ctxt->myDoc != NULL && ctxt->myDoc != doc) {
        xmlFreeDoc(ctxt->myDoc);
    }
    ctxt->myDoc = NULL;
    htmlFreeParserCtxt(ctxt);
    return doc;
}

void teds_free_doc(xmlDocPtr doc) {
    if (doc != NULL)
        xmlFreeDoc(doc);
}

xmlNodePtr teds_doc_root(xmlDocPtr doc) {
    return xmlDocGetRootElement(doc);
}

int teds_node_type(xmlNodePtr n) { return (int)n->type; }
xmlNodePtr teds_node_children(xmlNodePtr n) { return n->children; }
xmlNodePtr teds_node_next(xmlNodePtr n) { return n->next; }
const char *teds_node_name(xmlNodePtr n) { return (const char *)n->name; }
const char *teds_node_content(xmlNodePtr n) { return (const char *)n->content; }
int teds_node_has_ns(xmlNodePtr n) { return n->ns != NULL; }
/* libxml2 < 2.11 splits "o:p" into prefix "o" and name "p" even for HTML. */
const char *teds_node_ns_prefix(xmlNodePtr n) {
    return (n->ns != NULL) ? (const char *)n->ns->prefix : NULL;
}

/* lxml: _getNodeAttributeValue -> xmlGetNsProp(node, name, NULL). Caller frees. */
char *teds_get_prop(xmlNodePtr n, const char *name) {
    return (char *)xmlGetNsProp(n, (const xmlChar *)name, NULL);
}

void teds_free(void *p) {
    if (p != NULL)
        xmlFree(p);
}
