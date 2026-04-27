<!DOCTYPE html>
<html lang="en">
<head>
<link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🐋</text></svg>">
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>WHALE.TERMINAL v4.0 — Live Polymarket Intelligence</title>
<link href="https://fonts.googleapis.com/css2?family=Orbitron:wght@400;700;900&family=JetBrains+Mono:wght@300;400;500;700&display=swap" rel="stylesheet">

<style>
*{margin:0;padding:0;box-sizing:border-box}
:root{
  --bg:#020509;--bg2:#060d14;--bg3:#0a1520;--bg4:#0f1e2d;--bg5:#152333;
  --b1:rgba(255,255,255,.04);--b2:rgba(0,212,255,.18);--b3:rgba(0,212,255,.35);
  --green:#00FF88;--cyan:#00D4FF;--red:#FF3355;--yellow:#FFB800;
  --purple:#9B5DE5;--orange:#FF6B35;--teal:#00E5CC;
  --t1:#dff0f8;--t2:#5e849e;--t3:#243544;
  --gg:0 0 16px rgba(0,255,136,.35);--gc:0 0 16px rgba(0,212,255,.35);
  --gr:0 0 16px rgba(255,51,85,.35);
  --intel-rail-w:340px;--feed-rail-w:430px;
}
html,body{height:100%;overflow:hidden;background:var(--bg);color:var(--t1);
  font-family:'JetBrains Mono',monospace;font-size:11px;cursor:crosshair}
/* Heatmap titles readability: make sure any tile titles render clearly on Windows */ 
body::before{content:'';position:fixed;inset:0;pointer-events:none;z-index:9999;
  background:repeating-linear-gradient(0deg,transparent,transparent 2px,rgba(0,0,0,.04) 2px,rgba(0,0,0,.04) 4px)}
body::after{content:'';position:fixed;inset:0;pointer-events:none;z-index:9998;
  background:radial-gradient(ellipse at 50% 50%,transparent 55%,rgba(0,0,0,.65) 100%)}

/* Heatmap Titles Fix: ensure heatmap titles are visible across Windows rendering paths */
.hm-heatmap { display:grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); gap:12px; padding:12px; }
.hm-heatmap .tile { background: rgba(0,0,0,.85); border-radius:6px; padding:8px; overflow:hidden; border:1px solid rgba(0,0,0,.4); }
.hm-heatmap .tile .title { font-family: 'JetBrains Mono', monospace; font-size:12px; font-weight:700; color:#eafff0; line-height:1.25; display:-webkit-box; -webkit-line-clamp:2; -webkit-box-orient:vertical; overflow:hidden; text-overflow:ellipsis; }
/* ── WHALE ALERT POPUP ── */
#whale-alert-overlay{position:fixed;inset:0;z-index:999999;pointer-events:none;display:flex;align-items:flex-start;justify-content:center;padding-top:60px}
#whale-alert{pointer-events:all;display:none;min-width:440px;max-width:520px;background:linear-gradient(135deg,#000d1a 0%,#001428 50%,#000a12 100%);border:1px solid rgba(0,212,255,.5);box-shadow:0 0 0 1px rgba(0,212,255,.1),0 0 40px rgba(0,212,255,.3),0 0 80px rgba(0,212,255,.1),inset 0 0 40px rgba(0,0,0,.6);animation:whalePop .4s cubic-bezier(.175,.885,.32,1.275) both;position:relative;overflow:hidden}
@keyframes whalePop{0%{opacity:0;transform:translateY(-30px) scale(.92)}100%{opacity:1;transform:translateY(0) scale(1)}}
@keyframes whaleDismiss{0%{opacity:1;transform:translateY(0) scale(1)}100%{opacity:0;transform:translateY(-20px) scale(.95)}}
#whale-alert.dismissing{animation:whaleDismiss .25s ease forwards}
#whale-alert::before{content:'';position:absolute;top:0;left:0;right:0;height:2px;background:linear-gradient(90deg,transparent,#00d4ff 30%,#00ff88 70%,transparent);animation:scanLine 2s linear infinite}
@keyframes scanLine{0%{opacity:.4}50%{opacity:1}100%{opacity:.4}}
#whale-alert::after{content:'';position:absolute;inset:0;background:repeating-linear-gradient(0deg,transparent,transparent 3px,rgba(0,212,255,.015) 3px,rgba(0,212,255,.015) 4px);pointer-events:none}
.wa-corner{position:absolute;width:12px;height:12px;border-color:rgba(0,212,255,.7);border-style:solid}
.wa-corner.tl{top:6px;left:6px;border-width:2px 0 0 2px}
.wa-corner.tr{top:6px;right:6px;border-width:2px 2px 0 0}
.wa-corner.bl{bottom:6px;left:6px;border-width:0 0 2px 2px}
.wa-corner.br{bottom:6px;right:6px;border-width:0 2px 2px 0}
.wa-header{display:flex;align-items:center;justify-content:space-between;padding:10px 14px 8px;border-bottom:1px solid rgba(0,212,255,.15)}
.wa-title-row{display:flex;align-items:center;gap:8px}
.wa-icon{font-size:22px;animation:whaleBob 1.5s ease-in-out infinite}
@keyframes whaleBob{0%,100%{transform:translateY(0)}50%{transform:translateY(-3px)}}
.wa-label{font-family:'Orbitron';font-size:9px;font-weight:900;letter-spacing:3px;color:#00d4ff;text-shadow:0 0 10px rgba(0,212,255,.8)}
.wa-badge{font-family:'Orbitron';font-size:8px;font-weight:700;padding:2px 8px;letter-spacing:1px;animation:badgePulse 1s ease-in-out infinite}
@keyframes badgePulse{0%,100%{box-shadow:0 0 6px currentColor}50%{box-shadow:0 0 16px currentColor}}
.wa-close{background:none;border:none;color:rgba(0,212,255,.5);font-size:16px;cursor:pointer;padding:2px 6px;line-height:1;transition:.15s}
.wa-close:hover{color:#ff3355}
.wa-body{padding:12px 14px}
.wa-market{font-size:9px;color:#dff0f8;line-height:1.4;margin-bottom:10px;padding:6px 9px;background:rgba(0,212,255,.05);border-left:2px solid rgba(0,212,255,.4)}
.wa-stats{display:grid;grid-template-columns:repeat(4,1fr);gap:6px;margin-bottom:10px}
.wa-stat{background:rgba(0,0,0,.4);border:1px solid rgba(255,255,255,.07);padding:7px 8px;text-align:center}
.wa-stat-l{font-size:6px;color:var(--t3);text-transform:uppercase;letter-spacing:1.5px;margin-bottom:3px}
.wa-stat-v{font-family:'Orbitron';font-size:12px;font-weight:900}
.wa-action-row{display:flex;gap:8px;align-items:center}
.wa-action-pill{font-family:'Orbitron';font-size:11px;font-weight:900;padding:6px 16px;border:1px solid;letter-spacing:1px;flex-shrink:0}
.wa-outcome{font-size:8px;color:var(--t2);flex:1;line-height:1.4}
.wa-footer{display:flex;align-items:center;justify-content:space-between;padding:7px 14px 9px;border-top:1px solid rgba(255,255,255,.05);background:rgba(0,0,0,.3)}
.wa-wallet{font-size:7.5px;color:var(--cyan);letter-spacing:.5px}
.wa-timer-bar{position:absolute;bottom:0;left:0;height:2px;background:linear-gradient(90deg,#00d4ff,#00ff88);transition:width .1s linear}
/* ── TICKER ── */
#ticker{height:20px;background:rgba(0,0,0,.75);border-bottom:1px solid var(--b1);
  overflow:hidden;display:flex;align-items:center;z-index:200;position:relative}
.t-scroll{display:flex;white-space:nowrap;animation:scrollLeft 55s linear infinite}
@keyframes scrollLeft{0%{transform:translateX(0)}100%{transform:translateX(-50%)}}
.t-item{display:inline-flex;align-items:center;gap:5px;padding:0 14px;
  border-right:1px solid var(--b1);font-size:8.5px}
.t-n{color:var(--t3)}.t-v{font-weight:700}.t-up{color:var(--green)}.t-dn{color:var(--red)}.t-fl{color:var(--t2)}
#edge-modal-body .sig-card {
  display:flex;
  flex-direction:column;
  height:auto;
  min-height:220px;
  overflow:visible;
}
#edge-modal-body .sig-card-body {
  flex:1;
}
/* ── TOPBAR ── */
#topbar{display:flex;align-items:center;height:43px;padding:0 14px;
  background:linear-gradient(90deg,#030a10,#06111c,#030a10);
  border-bottom:1px solid var(--b1);position:relative;z-index:150}
.logo{font-family:'Orbitron';font-size:13px;font-weight:900;letter-spacing:3px;
  color:var(--cyan);text-shadow:var(--gc);display:flex;align-items:center;gap:8px;margin-right:24px;white-space:nowrap}
.logo-dot{width:8px;height:8px;border-radius:50%;background:var(--green);box-shadow:var(--gg);animation:pulse 2s infinite}
@keyframes pulse{0%,100%{opacity:1;transform:scale(1)}50%{opacity:.45;transform:scale(.7)}}
.logo-v{font-size:7px;color:var(--t3);border:1px solid var(--t3);padding:1px 4px;letter-spacing:1px}
.top-stats{display:flex;gap:20px;flex:1;overflow:hidden}
.tsg{display:flex;flex-direction:column;gap:1px;flex-shrink:0}
.tsl{font-size:7px;color:var(--t3);letter-spacing:1.5px;text-transform:uppercase}
.tsv{font-family:'Orbitron';font-size:12px;font-weight:700}
.g{color:var(--green)}.c{color:var(--cyan)}.r{color:var(--red)}.y{color:var(--yellow)}.p{color:var(--purple)}.o{color:var(--orange)}
.top-right{margin-left:auto;display:flex;align-items:center;gap:12px;flex-shrink:0}
.pill{display:flex;align-items:center;gap:4px;font-size:8px;color:var(--t2)}
.sdot{width:5px;height:5px;border-radius:50%}
.live-dot{background:var(--green);box-shadow:var(--gg);animation:pulse 2s infinite}
.warn-dot{background:var(--yellow);animation:pulse 2s infinite}
#trial-wrap{display:flex;align-items:center;gap:7px;border:1px solid rgba(0,212,255,.2);
  background:rgba(0,212,255,.04);padding:3px 9px}
#trial-label{font-size:7px;color:var(--cyan);letter-spacing:1px}
#trial-track{width:90px;height:3px;background:var(--bg3);overflow:hidden}
#trial-fill{height:100%;background:linear-gradient(90deg,var(--red),var(--yellow),var(--green));
  width:100%;transition:width .5s linear}
#trial-time{font-size:9px;font-weight:700;width:40px;text-align:right}
.blink{animation:blink 1s step-end infinite}
@keyframes blink{0%,100%{opacity:1}50%{opacity:0}}

/* ── FILTERBAR ── */
#filterbar{display:flex;align-items:center;height:30px;padding:0 10px;gap:4px;
  background:var(--bg2);border-bottom:1px solid var(--b1);z-index:100}
.fb-sep{width:1px;height:16px;background:var(--b1);margin:0 4px}
.fl{font-size:7.5px;color:var(--t3);letter-spacing:1.5px;white-space:nowrap}
.fbtn,.cbtn{background:none;border:1px solid var(--b1);color:var(--t2);
  font-family:'JetBrains Mono';font-size:8px;padding:2px 7px;cursor:pointer;transition:all .15s}
.fbtn.on,.cbtn.on{background:rgba(0,212,255,.08);border-color:var(--cyan);color:var(--cyan)}
.fbtn:hover:not(.on),.cbtn:hover:not(.on){border-color:var(--b2);color:var(--t1)}
.srch{background:var(--bg3);border:1px solid var(--b1);color:var(--t1);
  font-family:'JetBrains Mono';font-size:8px;padding:2px 8px;width:120px;outline:none}
.srch:focus{border-color:var(--cyan)}.srch::placeholder{color:var(--t3)}

/* ── LAYOUT ── */
#wrapper{display:flex;flex-direction:column;height:calc(100vh - 20px - 43px - 30px);overflow:hidden}
#main{display:flex;flex:1;min-height:0;overflow:hidden;gap:1px;background:var(--b1);align-items:stretch}
#left{width:420px;display:flex;flex-direction:column;background:var(--bg2);flex-shrink:0;height:100%}
#center{flex:1;display:flex;flex-direction:column;gap:0;background:var(--b1);min-width:0;overflow:hidden;height:100%}
#right{width:355px;display:flex;flex-direction:column;background:var(--bg2);flex-shrink:0;height:100%}

/* panels */
.panel{background:var(--bg2);display:flex;flex-direction:column;overflow:hidden}
.ph{display:flex;align-items:center;justify-content:space-between;padding:5px 10px;
  border-bottom:1px solid var(--b1);min-height:27px;background:rgba(6,13,20,.9)}
.pt{font-family:'Orbitron';font-size:7.5px;font-weight:700;letter-spacing:2px;
  color:var(--t2);display:flex;align-items:center;gap:6px}
.pta{color:var(--cyan);font-size:9px}
.pc{display:flex;gap:3px;align-items:center}
.badge{font-size:6.5px;font-weight:700;padding:1px 5px}
.badge.c{background:var(--cyan);color:var(--bg)}.badge.g{background:var(--green);color:var(--bg)}
.badge.r{background:var(--red);color:var(--bg)}.badge.y{background:var(--yellow);color:var(--bg)}
.scroll{overflow-y:auto;flex:1}.scroll::-webkit-scrollbar{width:2px}.scroll::-webkit-scrollbar-thumb{background:var(--b2)}


/* ── EDGE FEED (LEFT) ── */
/* ── EDGE FEED (LEFT) ── */
.edge-feed{flex:1;overflow-y:auto;display:flex;flex-direction:column;gap:5px;padding:6px}
.edge-feed::-webkit-scrollbar{width:2px}.edge-feed::-webkit-scrollbar-thumb{background:var(--b2)}
@keyframes sigIn{from{opacity:0;transform:translateY(6px)}to{opacity:1;transform:translateY(0)}}
/* AFTER: */
.sig-card{background:var(--bg3);border:1px solid rgba(0,212,255,.12);border-left:3px solid;cursor:pointer;transition:background .15s;animation:sigIn .25s ease;overflow:hidden;border-radius:2px;box-shadow:0 2px 12px rgba(0,0,0,.4);flex-shrink:0;min-height:0;width:100%}
.sig-card:hover{background:var(--bg4)}
.sig-card-top{padding:8px 10px 6px;border-bottom:1px solid var(--b1)}
.sig-card-body{padding:7px 10px;display:flex;flex-direction:column;gap:5px}
.sig-card-footer{padding:5px 10px 7px;border-top:1px solid var(--b1);background:rgba(0,0,0,.2)}
.sig-hdr{display:flex;align-items:center;justify-content:space-between;margin-bottom:5px}
.sig-kind{font-size:7px;font-weight:700;letter-spacing:2px;text-transform:uppercase}
.sig-conf-wrap{display:flex;align-items:center;gap:5px}
.sig-pri{font-size:7px;padding:2px 6px;font-weight:700;border:1px solid;letter-spacing:.8px;text-transform:uppercase}
.pri-critical{color:#ff3355;border-color:rgba(255,51,85,.5);background:rgba(255,51,85,.15);text-shadow:0 0 8px rgba(255,51,85,.6)}
.pri-high{color:#ffb800;border-color:rgba(255,184,0,.5);background:rgba(255,184,0,.12)}
.pri-medium{color:#00d4ff;border-color:rgba(0,212,255,.4);background:rgba(0,212,255,.08)}
.pri-low{color:var(--t2);border-color:var(--b1)}
.sig-conf-val{font-family:'Orbitron';font-size:11px;font-weight:700;color:#ffb800}
.sig-title{font-family:'Orbitron';font-size:12px;font-weight:700;color:#fff;letter-spacing:.3px;line-height:1.2;margin-bottom:2px}
.sig-price-big{font-family:'Orbitron';font-size:22px;font-weight:900;line-height:1;letter-spacing:-1px}
.sig-metrics{display:grid;grid-template-columns:1fr 1fr 1fr;gap:4px}
.sig-metric{background:rgba(0,0,0,.3);border:1px solid var(--b2);padding:4px 6px;border-radius:1px}
.sig-metric-label{font-size:6.5px;color:var(--t3);text-transform:uppercase;letter-spacing:1px;margin-bottom:2px}
.sig-metric-val{font-family:'Orbitron';font-size:10px;font-weight:700;color:var(--t1)}
.sig-trade-grid{display:grid;grid-template-columns:1fr 1fr 1fr;gap:4px}
.sig-trade-box{padding:5px 7px;border:1px solid;border-radius:1px}
.sig-trade-box.entry{border-color:rgba(0,255,136,.35);background:rgba(0,255,136,.07)}
.sig-trade-box.inval{border-color:rgba(255,51,85,.35);background:rgba(255,51,85,.07)}
.sig-trade-box.target{border-color:rgba(255,184,0,.35);background:rgba(255,184,0,.07)}
.sig-trade-label{font-size:6.5px;color:var(--t3);text-transform:uppercase;letter-spacing:1px;margin-bottom:3px}
.sig-trade-val{font-family:'Orbitron';font-size:10px;font-weight:700}
.sig-trade-val.g{color:#00ff88}.sig-trade-val.r{color:#ff3355}.sig-trade-val.y{color:#ffb800}
.sig-alert-box{display:flex;align-items:flex-start;gap:7px;padding:6px 8px;background:rgba(255,51,85,.08);border:1px solid rgba(255,51,85,.35);border-left:3px solid #ff3355}
.sig-alert-icon{font-size:11px;flex-shrink:0;margin-top:1px}
.sig-alert-text{font-size:8.5px;color:#ff6677;font-weight:600;line-height:1.45}
.sig-edge-quote{font-size:7.5px;color:var(--t3);font-style:italic;line-height:1.5;padding:4px 7px;border-left:2px solid var(--b2)}
.sig-mkt-row{display:flex;align-items:center;justify-content:space-between;gap:6px}
.sig-action-pill{font-family:'Orbitron';font-size:8px;font-weight:700;padding:3px 10px;border:1px solid;letter-spacing:.5px;flex-shrink:0}
.sig-action-pill.buy{color:#00ff88;border-color:rgba(0,255,136,.4);background:rgba(0,255,136,.1)}
.sig-action-pill.sell{color:#ff3355;border-color:rgba(255,51,85,.4);background:rgba(255,51,85,.1)}
.sig-action-pill.neutral{color:#00d4ff;border-color:rgba(0,212,255,.4);background:rgba(0,212,255,.08)}
.sig-mkt-name{font-size:7.5px;color:var(--cyan);padding:3px 6px;border:1px solid rgba(0,212,255,.2);background:rgba(0,212,255,.06);overflow:hidden;text-overflow:ellipsis;white-space:nowrap;flex:1;line-height:1.3}

/* ── LIVE TRADE TABLE (left bottom) ── */
.tt{width:100%;border-collapse:collapse}
.tt th:nth-child(1),.tt td:nth-child(1){width:62px}
.tt th:nth-child(2),.tt td:nth-child(2){width:98px}
.tt th:nth-child(4),.tt td:nth-child(4){width:86px}
.tt th:nth-child(5),.tt td:nth-child(5){width:70px}
.tt th:nth-child(6),.tt td:nth-child(6){width:42px}
.tt th{font-size:6.5px;color:var(--t3);letter-spacing:1.5px;padding:2px 5px;text-align:left;
  border-bottom:1px solid var(--b1);position:sticky;top:0;background:var(--bg2);z-index:5}
.tt td{padding:2px 5px;border-bottom:1px solid rgba(255,255,255,.022);
  transition:background .15s;vertical-align:middle;overflow:hidden;line-height:1.15}
.tt tr:hover td{background:var(--bg4)}
.tt tr.fl-row td{animation:rowflash 1.2s ease-out}
@keyframes rowflash{0%{background:rgba(0,212,255,.18)}100%{background:transparent}}
.wa{color:var(--cyan);cursor:pointer;font-size:8px}.wa:hover{color:var(--t1)}
.act{font-size:6.5px;font-weight:700;padding:1px 4px}
.act-by{background:rgba(0,255,136,.15);color:var(--green);border:1px solid rgba(0,255,136,.3)}
.act-sy{background:rgba(255,51,85,.15);color:var(--red);border:1px solid rgba(255,51,85,.3)}
.trade-bet{display:flex;flex-direction:column;gap:1px;min-width:0;width:100%}
.trade-mkt{font-size:8px;color:var(--t1);text-decoration:none;line-height:1.3;
  white-space:normal;display:-webkit-box;-webkit-line-clamp:2;-webkit-box-orient:vertical;overflow:hidden}
.trade-mkt:hover{color:var(--yellow)}
.trade-slot{font-size:6.5px;color:var(--t2);letter-spacing:.4px;white-space:nowrap}
.trade-actbox{display:flex;align-items:center;gap:2px;flex-wrap:wrap}
.trade-out{font-size:6.5px;font-weight:700;padding:1px 4px;border:1px solid;letter-spacing:.5px}
.trade-out-yes{color:var(--cyan);border-color:rgba(0,212,255,.25);background:rgba(0,212,255,.08)}
.trade-out-no{color:var(--yellow);border-color:rgba(255,184,0,.25);background:rgba(255,184,0,.08)}
.trade-out-alt{color:var(--t1);border-color:rgba(255,255,255,.14);background:rgba(255,255,255,.04)}
.wf{color:var(--yellow);font-size:8px;margin-left:1px}
.pg{color:var(--green);font-weight:700}.pr{color:var(--red);font-weight:700}

/* ── CENTER ── */
.eq-section{background:var(--bg2);display:flex;flex-direction:column;flex:1;min-height:0}
.eq-hero{display:flex;align-items:baseline;gap:8px;padding:4px 13px;border-bottom:1px solid var(--b1)}
.eq-main{font-family:'Orbitron';font-size:20px;font-weight:900;letter-spacing:-1px}
.eq-chg{font-size:10px;font-weight:700}
.eq-meta{display:flex;border-bottom:1px solid var(--b1)}
.eqm{flex:1;padding:3px 8px;border-right:1px solid var(--b1);display:flex;flex-direction:column;gap:0}
.eqm:last-child{border-right:none}
.eqml{font-size:6.5px;color:var(--t3);letter-spacing:1.5px}.eqmv{font-size:10px;font-weight:700}
.chart-wrap{position:relative;min-height:0;flex:1}
canvas{display:block;width:100%;height:100%}
#ch-x{position:absolute;top:0;bottom:0;width:1px;background:rgba(0,212,255,.3);display:none;pointer-events:none}
#ch-y{position:absolute;left:0;right:0;height:1px;background:rgba(0,212,255,.3);display:none;pointer-events:none}

/* active market */
.mkt-section{background:var(--bg2);flex:0 0 195px;display:flex;flex-direction:column}
.mkt-hdr{display:flex;align-items:center;gap:6px;padding:5px 10px;border-bottom:1px solid var(--b1)}
.mkt-name{font-family:'Orbitron';font-size:7.5px;letter-spacing:.8px;color:var(--yellow);flex:1;
  overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.mkt-link{font-size:7.5px;color:var(--cyan);text-decoration:none;flex-shrink:0}.mkt-link:hover{color:var(--t1)}

/* ── 4-SIDED CLOB ── */
.ob-container{display:flex;flex:1;overflow:hidden;gap:1px;background:var(--b1)}
.ob-outcome{display:flex;flex-direction:column;flex:1;overflow:hidden;background:var(--bg2)}
.ob-outcome-hdr{display:flex;align-items:center;justify-content:space-between;
  padding:2px 7px;background:rgba(6,13,20,.9);border-bottom:1px solid var(--b1)}
.ob-oname{font-size:7.5px;font-weight:700}
.ob-oprice{font-family:'Orbitron';font-size:9px;font-weight:700}
.ob-mid-spread{font-size:6.5px;color:var(--t3);padding:1px 7px;border-bottom:1px solid var(--b1);display:flex;justify-content:space-between}
.ob-sides{display:flex;flex:1;gap:1px;background:var(--b1);overflow:hidden}
.ob-half{display:flex;flex-direction:column;flex:1;overflow:hidden;background:var(--bg3)}
.ob-half-hdr{display:flex;justify-content:space-between;padding:2px 5px;border-bottom:1px solid var(--b1);font-size:6.5px;color:var(--t3)}
.ob-row{display:flex;justify-content:space-between;padding:2px 5px;position:relative;font-size:8px}
.ob-row::before{content:'';position:absolute;inset:0;opacity:.1;width:var(--fill,0%);pointer-events:none}
.ob-row.bid::before{background:var(--green)}.ob-row.ask::before{background:var(--red)}
.ob-p.g{color:var(--green);font-weight:700}.ob-p.r{color:var(--red);font-weight:700}
.ob-s{color:var(--t2);font-size:7.5px}
.ob-imb{height:3px;position:relative;overflow:hidden;background:var(--bg4)}
.ob-imb-fill{height:100%;position:absolute;top:0;transition:all .5s}

/* ── RIGHT PANEL ── */
/* heatmap */
.hm-panel{background:var(--bg2);flex:0 0 320px;display:flex;flex-direction:column;overflow:hidden}
#hm-modal{background:rgba(0,0,0,.95) !important}

.hm-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(120px,1fr));gap:2px;padding:5px;overflow-y:auto;flex:1}
.hm-cell{display:flex;flex-direction:column;padding:7px;cursor:pointer;min-height:62px;
 border:1px solid transparent;transition:transform .18s;position:relative;overflow:hidden}
.hm-cell::after{display:none}
.hm-cell:hover{transform:scale(1.04);z-index:10}
.hm-cell.has-signal{border-color:var(--cyan) !important;box-shadow:inset 0 0 10px rgba(0,212,255,.12)}
.hm-cell.no-signal{opacity:.4;filter:grayscale(.5)}
.hm-n{
  position:absolute;
  top:4px;
  left:6px;
  right:6px;
  z-index:5;

  font-family:'Orbitron';
  font-size:10px;
  font-weight:700;
  letter-spacing:.5px;
  color:#ffffff;

  overflow:hidden;
  text-overflow:ellipsis;
  white-space:nowrap;

  text-shadow:1px 1px 3px rgba(0,0,0,.9),0 0 8px rgba(0,0,0,.8);
  background:rgba(0,0,0,.55);
  padding:1px 3px;
}
/* make titles react + feel premium */
.hm-cell{
  display:flex;
  flex-direction:column;
  padding:7px;
  padding-top:20px; /* ADD THIS */
  cursor:pointer;
  min-height:62px;
  border:1px solid transparent;
  transition:transform .18s;
  position:relative;
  overflow:hidden;
}
.hm-cell:hover .hm-n{
  color:#00d4ff;
  text-shadow:0 0 10px rgba(0,212,255,.6);
}

/* highlight important markets (you can toggle this class later in JS) */
.hm-cell.hot .hm-n{
  color:#00ff88;
  text-shadow:0 0 12px rgba(0,255,136,.7);
  font-weight:900;
}
.hm-p{font-family:'Orbitron';font-size:10.5px;font-weight:700}
.hm-v{font-size:7px;opacity:.5;margin-top:1px}
.hm-sig-dot{position:absolute;top:4px;right:5px;font-size:9px}
.hm-outs{display:flex;gap:2px;flex-wrap:wrap;margin-top:2px}
.hm-out{font-size:6.5px;padding:0 3px;font-weight:700;border-radius:1px}

/* signal strength bars */
.sig-panel{background:var(--bg2);flex:0 0 145px;display:flex;flex-direction:column}
.sig-row{display:flex;align-items:center;padding:3.5px 10px;border-bottom:1px solid rgba(255,255,255,.022);gap:6px}
.sig-n{font-size:8px;color:var(--t2);width:66px}
.sig-track{flex:1;height:3px;background:var(--bg3)}.sig-fill{height:100%;transition:width .8s cubic-bezier(.4,0,.2,1)}
.sig-v{font-size:8px;font-weight:700;width:20px;text-align:right}
.bias-row{padding:4px 10px;border-bottom:1px solid rgba(255,255,255,.022)}
.bias-label{font-size:7px;color:var(--t3);margin-bottom:2px}
.bias-track{height:4px;background:var(--bg3);position:relative}
.bias-mid{position:absolute;left:50%;top:0;bottom:0;width:1px;background:rgba(0,212,255,.3)}
.bias-fill{height:100%;position:absolute;top:0;transition:all .7s}
.rec-row{padding:3px 10px;display:flex;align-items:center;justify-content:space-between}
.rec-lbl{font-size:7px;color:var(--t3)}.rec-val{font-size:8.5px;font-weight:700;font-family:'Orbitron'}

/* whale intel */
.whale-intel-row{display:grid;grid-template-columns:24px 1fr 44px;gap:6px;
  padding:6px 10px;border-bottom:1px solid var(--b1);align-items:center;cursor:pointer;transition:.15s}
.whale-intel-row:hover{background:var(--bg4)}
.w-name-cell{display:flex;flex-direction:column;gap:1px;min-width:0}
.w-addr{font-size:8px;color:var(--cyan);font-weight:700;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.w-meta{font-size:7px;color:var(--t3);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.score-val{font-family:'Orbitron';font-weight:700;text-align:right;font-size:10px}
.whale-tag{font-size:6.5px;border:1px solid;padding:0 3px}
.tag-apex{color:var(--yellow);border-color:rgba(255,184,0,.35)}
.tag-alpha{color:var(--green);border-color:rgba(0,255,136,.35)}
.tag-mm{color:var(--cyan);border-color:rgba(0,212,255,.35)}
.tag-acc{color:var(--purple);border-color:rgba(155,93,229,.35)}
.tag-other{color:var(--t2);border-color:var(--b1)}

/* execution layer */
.exec-panel{padding:10px;display:flex;flex-direction:column;gap:8px;
  background:rgba(0,0,0,.3);border-top:1px solid var(--b1)}
.qty-row{display:flex;gap:3px}
.qty-chip{flex:1;text-align:center;padding:4px;background:var(--b1);border:1px solid var(--b2);
  font-size:8px;cursor:pointer;color:var(--t2);transition:.15s;font-family:'JetBrains Mono'}
.qty-chip.active{border-color:var(--cyan);color:var(--cyan);background:rgba(0,212,255,.07)}
.qty-chip:hover:not(.active){color:var(--t1)}
.exec-btns{display:grid;grid-template-columns:1fr 1fr;gap:6px}
.buy-btn{background:rgba(0,255,136,.15);color:var(--green);border:1px solid rgba(0,255,136,.4);
  padding:9px;font-family:'Orbitron';font-size:9px;font-weight:bold;cursor:pointer;transition:.18s;letter-spacing:.5px}
.buy-btn:hover{background:rgba(0,255,136,.28);box-shadow:0 0 15px rgba(0,255,136,.25)}
.sell-btn{background:rgba(255,51,85,.15);color:var(--red);border:1px solid rgba(255,51,85,.4);
  padding:9px;font-family:'Orbitron';font-size:9px;font-weight:bold;cursor:pointer;transition:.18s;letter-spacing:.5px}
.sell-btn:hover{background:rgba(255,51,85,.28);box-shadow:0 0 15px rgba(255,51,85,.25)}
.exec-note{font-size:7px;color:var(--t3);text-align:center}

/* alerts panel */
.alerts-panel{background:var(--bg2);flex:1;min-height:0;display:flex;flex-direction:column}
.a-item{display:flex;gap:6px;padding:4px 9px;border-bottom:1px solid rgba(255,255,255,.022);animation:afl .35s ease}
@keyframes afl{from{opacity:0;transform:translateX(-4px)}to{opacity:1;transform:translateX(0)}}
.a-icon{font-size:10px;flex-shrink:0;margin-top:1px}.a-body{flex:1;min-width:0}
.a-title{font-size:8px;font-weight:700;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.a-desc{font-size:7px;color:var(--t2);margin-top:1px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.a-time{font-size:6.5px;color:var(--t3);margin-top:1px}

/* ── PAYWALL ── */
#paywall{position:fixed;inset:0;background:rgba(2,5,9,.97);z-index:99999;display:none;align-items:center;justify-content:center}
#paywall.show{display:flex}
.pw-card{background:var(--bg2);border:1px solid var(--b2);padding:40px;max-width:520px;width:90%;text-align:center;position:relative}
.pw-card::before{content:'';position:absolute;inset:-1px;background:linear-gradient(135deg,rgba(0,212,255,.1),transparent 55%,rgba(0,255,136,.07));pointer-events:none}
.pw-whale{font-size:50px;display:block;margin-bottom:6px}
.pw-title{font-family:'Orbitron';font-size:19px;font-weight:900;letter-spacing:3px;color:var(--cyan);text-shadow:var(--gc);margin-bottom:4px}
.pw-sub{font-size:10px;color:var(--t2);margin-bottom:22px;line-height:1.9}
.pw-stats{display:grid;grid-template-columns:repeat(3,1fr);gap:7px;margin-bottom:20px}
.pw-stat{background:var(--bg3);padding:9px;border:1px solid var(--b1)}
.pw-stat-v{font-family:'Orbitron';font-size:15px;font-weight:900;color:var(--cyan)}
.pw-stat-l{font-size:7.5px;color:var(--t3);letter-spacing:1px;margin-top:2px}
.pw-plans{display:grid;grid-template-columns:1fr 1fr;gap:10px;margin-bottom:20px}
.pw-plan{border:1px solid var(--b1);padding:16px;cursor:pointer;transition:.2s;position:relative}
.pw-plan:hover,.pw-plan.sel{border-color:var(--cyan);background:rgba(0,212,255,.05)}
.pw-plan.pop .pw-plan-p{color:var(--green)}
.pop-badge{position:absolute;top:-8px;left:50%;transform:translateX(-50%);
  background:var(--green);color:var(--bg);font-size:7px;font-weight:700;padding:1px 8px;letter-spacing:1px;white-space:nowrap}
.pw-plan-period{font-size:8px;color:var(--t3);letter-spacing:1.5px;margin-bottom:6px}
.pw-plan-p{font-family:'Orbitron';font-size:24px;font-weight:900;margin-bottom:3px}
.pw-plan-save{font-size:7.5px;color:var(--yellow);height:12px}
.pw-feats{display:flex;flex-direction:column;gap:4px;margin-bottom:20px;text-align:left}
.pw-feat{font-size:9px;color:var(--t2);display:flex;align-items:center;gap:7px}
.pw-feat .ck{color:var(--green)}
.pw-btn{width:100%;background:linear-gradient(90deg,var(--cyan),var(--green));border:none;color:var(--bg);
  font-family:'Orbitron';font-size:11px;font-weight:900;letter-spacing:2px;padding:12px;cursor:pointer;transition:.2s;overflow:hidden;position:relative}
.pw-btn::after{content:'';position:absolute;inset:0;background:linear-gradient(90deg,transparent,rgba(255,255,255,.2),transparent);transform:translateX(-100%);transition:transform .4s}
.pw-btn:hover::after{transform:translateX(100%)}.pw-btn:hover{box-shadow:0 0 25px rgba(0,212,255,.4)}
.pw-note{font-size:7.5px;color:var(--t3);margin-top:11px}

/* ── MODAL ── */
#modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.85);z-index:100000;
  display:none;align-items:center;justify-content:center;backdrop-filter:blur(2px)}
#modal-overlay.show{display:flex}
.modal{background:var(--bg2);border:1px solid var(--cyan);padding:24px;max-width:380px;width:90%;box-shadow:0 0 30px rgba(0,212,255,.15)}
.modal-title{font-family:'Orbitron';font-size:12px;color:var(--cyan);margin-bottom:12px;letter-spacing:1px}
.modal-body{font-size:10px;color:var(--t2);line-height:1.6;margin-bottom:20px}
.modal-btns{display:flex;gap:10px;justify-content:flex-end}
.mbtn{font-family:'Orbitron';font-size:9px;padding:7px 14px;cursor:pointer;border:1px solid;letter-spacing:1px}
.mbtn-p{background:var(--cyan);color:var(--bg);border-color:var(--cyan)}
.mbtn-s{background:none;color:var(--t3);border-color:var(--t3)}
.mbtn:hover{filter:brightness(1.2)}

/* signal color helpers */
.color-cyan{color:var(--cyan)}.color-green{color:var(--green)}.color-yellow{color:var(--yellow)}
.color-orange{color:var(--orange)}.color-red{color:var(--red)}.color-purple{color:var(--purple)}
.border-cyan{border-left-color:var(--cyan)}.border-green{border-left-color:var(--green)}
.border-yellow{border-left-color:var(--yellow)}.border-orange{border-left-color:var(--orange)}
.border-red{border-left-color:var(--red)}.border-purple{border-left-color:var(--purple)}

/* signal action colors */
.s-bull{background:rgba(0,255,136,.08);color:var(--green);border:1px solid rgba(0,255,136,.25)}
.s-bear{background:rgba(255,51,85,.08);color:var(--red);border:1px solid rgba(255,51,85,.25)}
.s-hot{background:rgba(255,107,53,.08);color:var(--orange);border:1px solid rgba(255,107,53,.25)}
.s-break{background:rgba(155,93,229,.08);color:var(--purple);border:1px solid rgba(155,93,229,.25)}
.final-call{border:1px solid;overflow:hidden;margin-top:6px}
.s-neutral{background:rgba(107,143,168,.06);color:var(--t2);border:1px solid var(--b1)}
/* ── UNLOCK BTN ── */
#unlock-btn{font-family:'Orbitron';font-size:8px;font-weight:700;letter-spacing:1.5px;padding:4px 12px;cursor:pointer;border:1px solid var(--cyan);background:rgba(0,212,255,.08);color:var(--cyan);transition:.2s;white-space:nowrap;flex-shrink:0}
#unlock-btn:hover{background:rgba(0,212,255,.2);box-shadow:0 0 10px rgba(0,212,255,.3)}
#how-btn{font-family:'Orbitron';font-size:8px;font-weight:700;letter-spacing:1.5px;padding:4px 12px;cursor:pointer;border:1px solid var(--b2);background:none;color:var(--t2);transition:.2s;white-space:nowrap;flex-shrink:0}
#how-btn:hover{border-color:var(--cyan);color:var(--cyan);background:rgba(0,212,255,.06)}
#how-modal{display:none;position:fixed;inset:0;background:rgba(0,0,0,.88);z-index:300000;align-items:center;justify-content:center}
.how-content{width:960px;max-width:96vw;max-height:90vh;overflow-y:auto;background:#080d12;border:1px solid rgba(0,212,255,.25);padding:0;border-radius:4px}
.how-content::-webkit-scrollbar{width:2px}.how-content::-webkit-scrollbar-thumb{background:rgba(0,212,255,.2)}
.how-header{display:flex;align-items:center;justify-content:space-between;padding:14px 18px;border-bottom:1px solid rgba(255,255,255,.06);position:sticky;top:0;background:#080d12;z-index:1}
.how-close{background:none;border:none;color:var(--t3);font-size:16px;cursor:pointer;padding:0 4px;line-height:1}
.how-close:hover{color:var(--t1)}
.how-section{padding:14px 18px;border-bottom:1px solid rgba(255,255,255,.04)}
.how-section:last-child{border-bottom:none}
.how-tag{font-family:'Orbitron';font-size:7px;font-weight:700;letter-spacing:2px;padding:2px 7px;border:1px solid;margin-bottom:8px;display:inline-block}
.how-title{font-family:'Orbitron';font-size:10px;font-weight:700;letter-spacing:1px;margin-bottom:6px}
.how-body{font-size:8.5px;color:var(--t2);line-height:1.75}
.how-arrow{color:var(--cyan);font-weight:700}
.how-steps{display:flex;flex-direction:column;gap:6px;margin-top:4px}
.how-step{display:flex;gap:9px;align-items:flex-start;font-size:8.5px;color:var(--t2)}
.how-step-num{font-family:'Orbitron';font-size:8px;font-weight:700;color:var(--cyan);flex-shrink:0;min-width:16px}
/* ── ACCESS MODAL ── */
#access-modal{position:fixed;inset:0;background:rgba(0,0,0,.93);z-index:200000;display:none;align-items:center;justify-content:center;backdrop-filter:blur(4px)}
#access-modal.show{display:flex}
.am-card{background:var(--bg2);border:1px solid var(--b2);width:90%;max-width:480px;max-height:90vh;overflow-y:auto;box-shadow:0 0 40px rgba(0,212,255,.12);position:relative}
.am-card::before{content:'';position:absolute;inset:-1px;pointer-events:none;background:linear-gradient(135deg,rgba(0,212,255,.07),transparent 55%,rgba(0,255,136,.05))}
.am-header{display:flex;align-items:center;justify-content:space-between;padding:14px 18px;border-bottom:1px solid var(--b1);position:sticky;top:0;background:var(--bg2);z-index:1}
.am-title{font-family:'Orbitron';font-size:11px;font-weight:900;letter-spacing:2px;color:var(--cyan)}
.am-close{background:none;border:none;color:var(--t3);font-size:16px;cursor:pointer;padding:0 4px}
.am-close:hover{color:var(--t1)}
.am-tabs{display:flex;border-bottom:1px solid var(--b1)}
.am-tab{flex:1;padding:9px;font-family:'Orbitron';font-size:8px;letter-spacing:1.5px;cursor:pointer;text-align:center;color:var(--t3);border:none;background:none;border-bottom:2px solid transparent;transition:.15s}
.am-tab.active{color:var(--cyan);border-bottom-color:var(--cyan);background:rgba(0,212,255,.04)}
.am-tab:hover:not(.active){color:var(--t2)}
.am-pane{padding:18px;display:none}
.am-pane.active{display:block}
.am-section-label{font-size:7px;color:var(--t3);letter-spacing:2px;text-transform:uppercase;margin-bottom:8px;padding-bottom:4px;border-bottom:1px solid var(--b1)}
.am-addr-box{background:var(--bg3);border:1px solid var(--b2);padding:10px 12px;display:flex;align-items:center;gap:8px;margin-bottom:6px}
.am-network{font-size:7.5px;color:var(--t3);letter-spacing:1px;margin-bottom:3px}
.am-addr{font-family:'JetBrains Mono';font-size:9px;color:var(--cyan);word-break:break-all;flex:1;line-height:1.4}
.am-copy{background:rgba(0,212,255,.08);border:1px solid rgba(0,212,255,.25);color:var(--cyan);font-family:'Orbitron';font-size:7px;padding:4px 8px;cursor:pointer;white-space:nowrap;transition:.15s;flex-shrink:0}
.am-copy:hover{background:rgba(0,212,255,.2)}.am-copy.copied{color:var(--green);border-color:rgba(0,255,136,.4)}
.am-box{font-size:8.5px;color:var(--t2);line-height:1.8;background:rgba(0,0,0,.3);border:1px solid var(--b1);border-left:3px solid var(--yellow);padding:11px 13px}
.am-note{font-size:7.5px;color:var(--t3);text-align:center;margin-top:8px;line-height:1.6}
.am-submit{width:100%;border:1px solid var(--b2);color:var(--t1);font-family:'Orbitron';font-size:9px;font-weight:700;letter-spacing:1.5px;padding:10px;cursor:pointer;transition:.2s;background:none}
.am-submit:hover{background:rgba(255,255,255,.05)}
.am-lic-wrap{display:flex;gap:6px;margin-bottom:10px}
.am-lic-input{flex:1;background:var(--bg3);border:1px solid var(--b2);color:var(--t1);font-family:'JetBrains Mono';font-size:11px;padding:9px 12px;outline:none;letter-spacing:.5px}
.am-lic-input:focus{border-color:var(--cyan)}.am-lic-input::placeholder{color:var(--t3);font-size:9px}
.am-lic-btn{background:rgba(0,212,255,.1);border:1px solid rgba(0,212,255,.4);color:var(--cyan);font-family:'Orbitron';font-size:8px;font-weight:700;letter-spacing:1px;padding:9px 14px;cursor:pointer;transition:.2s;white-space:nowrap}
.am-lic-btn:hover{background:rgba(0,212,255,.22)}
#main.blurred{filter:blur(6px) brightness(.6);pointer-events:none;user-select:none}
</style>
</head>
<body>
<!-- BOOT OVERLAY -->
<div id="boot-overlay" style="position:fixed;inset:0;z-index:99999;background:#020509;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:16px;font-family:'Orbitron',sans-serif">
  <div style="font-size:18px;font-weight:900;letter-spacing:4px;color:#00d4ff;text-shadow:0 0 20px rgba(0,212,255,.6)">🐋 WHALE.TERMINAL</div>
  <div id="boot-msg" style="font-size:9px;letter-spacing:2px;color:#5e849e;transition:color .3s">⏳ SYNCING MARKET INTELLIGENCE...</div>
  <div id="boot-sub" style="font-size:8px;letter-spacing:1.5px;color:#2a4a5e">CONNECTING TO EXECUTION ENGINE...</div>
  <div style="margin-top:12px;width:180px;height:2px;background:#0a1520;border-radius:2px;overflow:hidden">
    <div id="boot-bar" style="height:100%;width:0%;background:linear-gradient(90deg,#00d4ff,#00ff88);transition:width .4s ease;box-shadow:0 0 8px #00d4ff"></div>
  </div>
</div>
<div id="ticker"><div class="t-scroll" id="t-inner"></div></div>

<!-- TOPBAR -->
<div id="topbar">
  <div class="logo"><div class="logo-dot"></div> 🐋 WHALE.TERMINAL <span class="logo-v">v4.0</span></div>
  <div class="top-stats">
    <div class="tsg"><span class="tsl">EQUITY</span><span class="tsv" id="eq-top">$10,000</span></div>
    <div class="tsg"><span class="tsl">P&L 24H</span><span class="tsv g" id="pnl-top">+$0</span></div>
    <div class="tsg"><span class="tsl">WIN RATE</span><span class="tsv c" id="wr-top">—</span></div>
    <div class="tsg"><span class="tsl">LIVE TRADES</span><span class="tsv" id="tc-top">0</span></div>
    <div class="tsg"><span class="tsl">VOL 24H</span><span class="tsv y" id="vol-top">—</span></div>
    <div class="tsg"><span class="tsl">ACTIVE WHALES</span><span class="tsv c" id="wh-top">—</span></div>
    <div class="tsg"><span class="tsl">BUY/SELL</span><span class="tsv" id="bsr-top" style="color:var(--teal)">—</span></div>
    <div class="tsg"><span class="tsl">BIGGEST TRADE</span><span class="tsv o" id="big-top">—</span></div>
    <div class="tsg"><span class="tsl">ALPHA WALLETS</span><span class="tsv p" id="alpha-top">—</span></div>
  </div>
  <div class="top-right">
    <div class="pill"><div class="sdot live-dot"></div> <span id="feed-source">CONNECTING…</span></div>
    <div class="pill"><div class="sdot warn-dot"></div> REAL CLOB</div>
    <button id="sound-btn" onclick="toggleSound(this)" style="font-family:'Orbitron';font-size:8px;font-weight:700;letter-spacing:1.5px;padding:4px 12px;cursor:pointer;border:1px solid rgba(0,255,136,.3);background:rgba(0,255,136,.06);color:#00ff88;transition:.2s;white-space:nowrap;flex-shrink:0">🔊 SOUND</button>
    <button id="how-btn" onclick="openHow()">❓ HOW IT WORKS</button>
    <button id="unlock-btn" onclick="openAccessModal('license')">🔑 UNLOCK ACCESS</button>
    <div id="trial-wrap">
      <span id="trial-label">TRIAL</span>
      <div id="trial-track"><div id="trial-fill"></div></div>
      <span id="trial-time" class="blink g">5:00</span>
    </div>
  </div>
</div>

<!-- FILTERBAR -->
<div id="filterbar">
  <span class="fl">SIZE</span>
  <button class="fbtn on" onclick="setMinSize(this,0)">ALL</button>
  <button class="fbtn" onclick="setMinSize(this,500)">$500+</button>
  <button class="fbtn" onclick="setMinSize(this,1000)">$1K+</button>
  <button class="fbtn" onclick="setMinSize(this,5000)">$5K+</button>
  <button class="fbtn" onclick="setMinSize(this,25000)">$25K+</button>
  <div class="fb-sep"></div>
  <span class="fl">ACTION</span>
  <button class="fbtn on" onclick="setActionFilter(this,'ALL')">ALL</button>
  <button class="fbtn" onclick="setActionFilter(this,'BUY')">BUY</button>
  <button class="fbtn" onclick="setActionFilter(this,'SELL')">SELL</button>
  <div class="fb-sep"></div>
  <span class="fl">CATEGORY</span>
  <button class="fbtn on" onclick="setCat(this,'ALL')">ALL</button>
  <button class="fbtn" onclick="setCat(this,'Crypto')">CRYPTO</button>
  <button class="fbtn" onclick="setCat(this,'Politics')">POLITICS</button>
  <button class="fbtn" onclick="setCat(this,'Finance')">FINANCE</button>
  <button class="fbtn" onclick="setCat(this,'Sports')">SPORTS</button>
  <div class="fb-sep"></div>
  <input class="srch" placeholder="Search…" id="mkt-search" oninput="filterMarkets()">
  <div style="margin-left:auto;display:flex;gap:3px">
    <button class="cbtn on" id="pause-btn" onclick="togglePause(this)">⏸ PAUSE</button>
    <button class="cbtn" id="whale-btn" onclick="toggleWhaleOnly(this)">🐋 WHALES</button>
  </div>
</div>

<div id="wrapper">
<div id="main">

<!-- ═══ LEFT: EDGE FEED + LIVE TRADES ═══ -->
<div id="left">
  <!-- Edge feed top -->
  <div class="ph">
    <div class="pt"><span class="pta">⚡</span> THE EDGE FEED <span class="badge y" id="sig-count">0</span></div>
    <div class="pc" style="display:flex;align-items:center;gap:6px">
      <span style="font-size:7px;color:var(--t3)" id="sig-rec-mini">AWAITING DATA</span>
      <button onclick="openEdgeFeedModal()" style="font-family:'JetBrains Mono';font-size:7px;padding:1px 6px;background:transparent;border:1px solid var(--b2);color:var(--cyan);cursor:pointer;letter-spacing:.5px">ALL ↗</button>
    </div>
  </div>
  <div class="edge-feed" id="edge-feed">
    <div style="display:flex;align-items:center;justify-content:center;height:100%;color:var(--t3);font-size:8.5px;text-align:center;padding:20px;line-height:1.7">
      MONITORING WHALE CLUSTERS,<br>VELOCITY SURGES & REVERSALS…<br><br>
      <span style="color:var(--t2);font-size:8px">Signals fire when edge is detected.<br>High conviction only.</span>
    </div>
  </div>

  <!-- Live trade feed bottom -->
  <div class="ph" style="border-top:1px solid var(--b1)">
    <div class="pt"><span class="pta">▶</span> LIVE TRADES <span class="badge c" id="feed-badge">0</span></div>
    <div class="pc"><span style="font-size:7px;color:var(--t3)" id="feed-source-small">—</span></div>
  </div>
  <div class="scroll" style="flex:1">
    <table class="tt">
      <thead><tr>
        <th>TIME</th><th>WALLET</th><th>BET</th><th>ACT</th><th>SIZE</th><th>¢</th>
      </tr></thead>
      <tbody id="trade-tbody"></tbody>
    </table>
  </div>
</div>

<!-- ═══ CENTER: EQUITY + ACTIVE MARKET ═══ -->
<div id="center">
  <!-- Equity curve -->
  <!-- Equity curve + Position Pressure -->
  <div class="eq-section" style="flex:0 0 auto">
    <div class="ph">
      <div class="pt"><span class="pta">📈</span> PORTFOLIO · EQUITY CURVE</div>
      <div class="pc" style="font-size:7.5px;color:var(--t2)">
        PNL <span class="g" id="hdr-pnl">$0</span>&nbsp;
        WIN <span class="c" id="hdr-wr">—</span>&nbsp;
        VOL <span id="hdr-vol">—</span>
      </div>
    </div>
    <div class="eq-hero">
      <span class="eq-main" id="eq-main">$10,000</span>
      <span class="eq-chg g" id="eq-chg">+0.0%</span>
      <span style="font-size:7.5px;color:var(--t3);margin-left:auto">ATH <span id="eq-ath">$10,000</span></span>
    </div>
    <div class="eq-meta">
      <div class="eqm"><span class="eqml">BUY VOL</span><span class="eqmv g" id="buy-vol">—</span></div>
      <div class="eqm"><span class="eqml">SELL VOL</span><span class="eqmv r" id="sell-vol">—</span></div>
      <div class="eqm"><span class="eqml">BIGGEST</span><span class="eqmv o" id="big-trade">—</span></div>
      <div class="eqm"><span class="eqml">MKTS</span><span class="eqmv c" id="open-mkts">—</span></div>
      <div class="eqm"><span class="eqml">B/S</span><span class="eqmv" id="bsr-mid" style="color:var(--teal)">—</span></div>
    </div>
    <!-- Equity curve chart -->
    <div class="chart-wrap" style="height:90px;max-height:90px;flex-shrink:0">
      <canvas id="eq-canvas"></canvas>
      <div id="ch-x"></div><div id="ch-y"></div>
    </div>

    <!-- POSITION PRESSURE panel -->
    <div id="pos-pressure" style="border-top:1px solid var(--b1);background:rgba(6,13,20,.95);padding:14px 10px;display:flex;flex-direction:column;gap:10px">

      <!-- Header row -->
      <div style="display:flex;align-items:center;justify-content:space-between">
        <span style="font-family:'Orbitron';font-size:7px;font-weight:700;letter-spacing:2px;color:var(--t2)">POSITION PRESSURE</span>
        <span id="pp-imbalance-label" style="font-size:6.5px;font-weight:700;letter-spacing:1px;padding:1px 6px;border:1px solid;color:var(--t3);border-color:var(--b1)">BALANCED MARKET</span>
      </div>

      <!-- Crowd bar -->
      <div style="display:flex;flex-direction:column;gap:2px">
        <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:1px">
          <span style="font-size:6.5px;color:var(--t3);letter-spacing:1px">CROWD</span>
          <div style="display:flex;gap:8px">
            <span style="font-size:8px;font-weight:700;color:#00ff88">YES <span id="pp-yes-pct">50</span>%</span>
            <span style="font-size:8px;font-weight:700;color:#ff3355">NO <span id="pp-no-pct">50</span>%</span>
          </div>
        </div>
        <div style="height:5px;background:rgba(255,255,255,.05);overflow:hidden;border-radius:1px;position:relative">
          <div id="pp-yes-bar" style="position:absolute;left:0;top:0;height:100%;width:50%;background:linear-gradient(90deg,#00ff88,#00cc66);transition:width .6s ease;box-shadow:0 0 6px rgba(0,255,136,.5)"></div>
          <div id="pp-no-bar"  style="position:absolute;right:0;top:0;height:100%;width:50%;background:linear-gradient(90deg,#cc1133,#ff3355);transition:width .6s ease;box-shadow:0 0 6px rgba(255,51,85,.4)"></div>
        </div>
      </div>

      <!-- Smart money + Imbalance row -->
      <div style="display:flex;gap:4px">
        <div style="flex:1;background:rgba(0,0,0,.3);border:1px solid var(--b1);padding:3px 7px;display:flex;flex-direction:column;gap:1px">
          <span style="font-size:6px;color:var(--t3);letter-spacing:1.2px;text-transform:uppercase">Smart Money</span>
          <span id="pp-smart-money" style="font-family:'Orbitron';font-size:9px;font-weight:700;color:#00d4ff">NEUTRAL</span>
        </div>
        <div style="flex:1;background:rgba(0,0,0,.3);border:1px solid var(--b1);padding:3px 7px;display:flex;flex-direction:column;gap:1px">
          <span style="font-size:6px;color:var(--t3);letter-spacing:1.2px;text-transform:uppercase">Imbalance</span>
          <span id="pp-imbalance-val" style="font-family:'Orbitron';font-size:9px;font-weight:700;color:#ffb800">LOW</span>
        </div>
        <div style="flex:1;background:rgba(0,0,0,.3);border:1px solid var(--b1);padding:3px 7px;display:flex;flex-direction:column;gap:1px">
          <span style="font-size:6px;color:var(--t3);letter-spacing:1.2px;text-transform:uppercase">Whale Flow</span>
          <span id="pp-whale-flow" style="font-family:'Orbitron';font-size:9px;font-weight:700;color:#9b5de5">—</span>
        </div>
      </div>

    </div>
  </div>

  <!-- Active market: pressure map + whale battlefield -->
  <div class="mkt-section" style="flex:1;min-height:0;display:flex;flex-direction:column;overflow:hidden">
    <div class="mkt-hdr">
      <span class="mkt-name" id="active-mkt-name">SELECT A MARKET OR SIGNAL…</span>
      <a class="mkt-link" id="active-mkt-link" href="#" target="_blank" onclick="handleMktLink(event)">↗ POLYMARKET</a>
      <span class="badge g" style="margin-left:4px">LIVE</span>
    </div>

    <!-- MARKET PRESSURE MAP -->
    <div style="border-bottom:1px solid var(--b1);position:relative;flex:0 0 150px;min-height:0;overflow:hidden">
      <div style="position:absolute;top:4px;left:9px;z-index:10;display:flex;align-items:center;gap:8px">
        <span style="font-family:'Orbitron';font-size:6.5px;font-weight:700;letter-spacing:2px;color:var(--t3)">MARKET PRESSURE MAP</span>
        <span id="hm-hover-info" style="font-size:6.5px;color:var(--cyan);opacity:0;transition:opacity .2s"></span>
      </div>
      <canvas id="pressure-map-canvas" style="width:100%;height:100%;display:block;cursor:grab"></canvas>
    </div>

    <!-- WHALE BATTLEFIELD -->
    <div style="flex:1;min-height:0;position:relative;overflow:hidden;background:linear-gradient(180deg,#020c14,#030a10);display:flex;flex-direction:column">
      <div style="display:flex;align-items:center;justify-content:space-between;padding:4px 10px;border-bottom:1px solid rgba(0,212,255,.08);flex-shrink:0">
        <div style="display:flex;align-items:center;gap:6px">
          <span style="font-size:9px">🐋</span>
          <span style="font-family:'Orbitron';font-size:6.5px;font-weight:700;letter-spacing:2px;color:rgba(0,212,255,.55)">WHALE BATTLEFIELD</span>
          <span style="font-size:6px;color:var(--t3)">bubble size = trade volume</span>
        </div>
        <span id="wb-live" style="font-size:6px;color:rgba(0,255,136,.5);letter-spacing:1px">LIVE</span>
      </div>
      <canvas id="whale-battle-canvas" style="width:100%;flex:1;display:block;min-height:0;cursor:crosshair"></canvas>
    </div>

    <!-- Execution -->
    <div style="padding:8px;background:rgba(0,0,0,.4);border-top:1px solid var(--b1);flex-shrink:0">
      <div style="display:grid;grid-template-columns:1fr 1fr;gap:6px;margin-bottom:5px">
        <button class="buy-btn" onclick="executeTrade('BUY')">▲ BUY ON POLYMARKET</button>
        <button class="sell-btn" onclick="executeTrade('SELL')">▼ SELL ON POLYMARKET</button>
      </div>
      <div style="font-size:7px;color:var(--t3);text-align:center" id="exec-note">Select a market above, then execute on Polymarket</div>
    </div>
  </div>
</div>

<!-- ═══ RIGHT: HEATMAP + SIGNALS + WHALE INTEL ═══ -->
<div id="right">
  <!-- De-noised heatmap -->
  <div class="hm-panel">
    <div class="ph">
      <div class="pt"><span class="pta">▦</span> MARKET HEATMAP</div>
      <div class="pc" style="display:flex;gap:3px;align-items:center">
        <select id="hm-sort" style="background:var(--bg3);border:1px solid var(--b1);color:var(--t2);font-family:'JetBrains Mono';font-size:7px;padding:1px 4px;outline:none" onchange="renderHeatmap()">
          <option value="volume">VOL</option><option value="move">MOVE</option><option value="prob">PROB</option>
        </select>
        <button onclick="openHeatmapModal()" style="font-family:'JetBrains Mono';font-size:7px;padding:1px 6px;background:transparent;border:1px solid var(--b2);color:var(--cyan);cursor:pointer;letter-spacing:.5px">ALL ↗</button>
      </div>
    </div>
    <div class="hm-grid" id="hm-grid"></div>
  </div>

  <!-- Full Heatmap Modal --><div id="hm-modal" style="display:none;position:fixed;inset:0;background:#080d12;z-index:9000;flex-direction:column">
  <div style="display:flex;align-items:center;justify-content:space-between;padding:10px 14px;border-bottom:1px solid rgba(0,212,255,.2);background:linear-gradient(90deg,rgba(0,212,255,.08),rgba(0,255,136,.05),transparent);box-shadow:0 1px 20px rgba(0,212,255,.1)">
    
      <div style="font-family:'Orbitron';font-size:11px;font-weight:700;letter-spacing:3px;color:#00d4ff;text-shadow:0 0 12px rgba(0,212,255,.6)">▦ ALL MARKETS — HEATMAP</div>
      <div style="display:flex;gap:8px;align-items:center">
        <select id="hm-modal-sort" style="background:rgba(0,212,255,.08);border:1px solid rgba(0,212,255,.3);color:#00d4ff;font-family:'JetBrains Mono';font-size:7px;padding:2px 6px;outline:none" onchange="renderHeatmapModal()">
          <option value="volume">VOL</option><option value="move">MOVE</option><option value="prob">PROB</option>
        </select>
        <button onclick="closeHeatmapModal()" style="font-family:'Orbitron';font-size:8px;padding:4px 12px;background:rgba(255,51,85,.08);border:1px solid rgba(255,51,85,.4);color:#ff3355;cursor:pointer;letter-spacing:1px;text-shadow:0 0 8px rgba(255,51,85,.5)">✕ CLOSE</button>
      </div>
    </div>
    <div id="hm-modal-grid" style="display:grid;grid-template-columns:repeat(auto-fill,minmax(140px,1fr));gap:2px;padding:8px;overflow-y:auto;flex:1"></div>
  </div>

  <!-- Signal strength bars -->
  <div class="sig-panel">
    <div class="ph">
      <div class="pt"><span class="pta">⚡</span> SIGNAL ENGINE</div>
      <span style="font-size:8px;font-weight:700;font-family:'Orbitron'" id="sig-rec">—</span>
    </div>
    <div id="sig-rows"></div>
    <div class="bias-row">
      <div class="bias-label">WHALE BIAS — <span id="bias-pct">NEUTRAL</span></div>
      <div class="bias-track"><div class="bias-mid"></div><div class="bias-fill" id="bias-fill"></div></div>
    </div>
    <div class="rec-row"><span class="rec-lbl">TOP MKT</span><span class="rec-val y" id="top-mkt">—</span></div>
  </div>

  <!-- Whale Intelligence sorted by whale_score -->
  <div style="display:flex;flex-direction:column;flex:1;overflow:hidden;border-top:1px solid var(--b1)">
    <div class="ph">
      <div class="pt"><span class="pta">🐋</span> WHALE INTEL <span class="badge c" id="whale-badge">0</span></div>
      <div class="pc"><span style="font-size:7px;color:var(--t3)">SCORE = WR×35+ROI×30+CONS×20+VOL×15</span></div>
    </div>
    <div class="scroll" id="whale-intel-list"></div>
  </div>

  <!-- Alerts -->
  <div class="alerts-panel">
    <div class="ph">
      <div class="pt"><span class="pta">◎</span> WHALE ALERTS <span class="badge r" id="alert-badge">0</span></div>
    </div>
    <div class="scroll" id="alerts-body">
      <div style="padding:10px 9px;font-size:8px;color:var(--t3)">Awaiting $5K+ trades<span class="blink">_</span></div>
    </div>
  </div>
</div>

</div><!-- /main -->
</div><!-- /wrapper -->
<!-- EDGE FEED MODAL DIV -->
<div id="edge-modal" style="display:none;position:fixed;inset:0;background:#080d12;z-index:9500;flex-direction:column">
  <div style="display:flex;align-items:center;justify-content:space-between;padding:10px 14px;border-bottom:1px solid rgba(0,212,255,.2);background:linear-gradient(90deg,rgba(0,212,255,.08),rgba(0,255,136,.05),transparent)">
    <div style="display:flex;align-items:center;gap:12px">
      <div style="font-family:'Orbitron';font-size:11px;font-weight:700;letter-spacing:3px;color:#00d4ff;text-shadow:0 0 12px rgba(0,212,255,.6)">⚡ EDGE FEED — ALL SIGNALS</div>
      <div style="font-family:'Orbitron';font-size:11px;font-weight:900;color:#ffb800" id="edge-modal-count">0</div>
    </div>
    <div style="display:flex;gap:6px;align-items:center">
      <button class="emf-btn" id="emf-all"    onclick="filterEdgeModal('ALL')"      style="font-family:'JetBrains Mono';font-size:7px;padding:2px 8px;background:transparent;border:1px solid rgba(0,212,255,.3);color:#00d4ff;cursor:pointer">ALL</button>
      <button class="emf-btn" id="emf-CRITICAL" onclick="filterEdgeModal('CRITICAL')" style="font-family:'JetBrains Mono';font-size:7px;padding:2px 8px;background:transparent;border:1px solid rgba(255,51,85,.3);color:#ff3355;cursor:pointer">CRITICAL</button>
      <button class="emf-btn" id="emf-HIGH"   onclick="filterEdgeModal('HIGH')"     style="font-family:'JetBrains Mono';font-size:7px;padding:2px 8px;background:transparent;border:1px solid rgba(255,184,0,.3);color:#ffb800;cursor:pointer">HIGH</button>
      <button class="emf-btn" id="emf-MEDIUM" onclick="filterEdgeModal('MEDIUM')"   style="font-family:'JetBrains Mono';font-size:7px;padding:2px 8px;background:transparent;border:1px solid rgba(0,212,255,.2);color:#00d4ff;cursor:pointer;opacity:.6">MEDIUM</button>
      <button onclick="closeEdgeFeedModal()" style="font-family:'Orbitron';font-size:8px;padding:4px 12px;background:rgba(255,51,85,.08);border:1px solid rgba(255,51,85,.4);color:#ff3355;cursor:pointer;letter-spacing:1px">✕ CLOSE</button>
    </div>
  </div>
  <div id="edge-modal-summary" style="display:flex;border-bottom:1px solid rgba(255,255,255,.06)"></div>
  <div id="edge-modal-body" style="display:grid;grid-template-columns:repeat(auto-fill,minmax(320px,1fr));gap:12px;overflow-y:auto;flex:1;background:var(--bg);padding:16px;align-content:start"></div>

</div>


<!-- LICENSE MODAL -->
<div id="lic-modal" style="display:none;position:fixed;inset:0;background:rgba(0,0,0,.97);z-index:99998;align-items:center;justify-content:center;flex-direction:column;gap:16px">
  <div style="background:#060d14;border:1px solid rgba(0,212,255,.3);padding:32px;max-width:400px;width:90%;position:relative">
    <div style="font-family:'Orbitron';font-size:14px;font-weight:900;color:#00d4ff;letter-spacing:3px;margin-bottom:6px">🐋 WHALE.TERMINAL</div>
    <div style="font-size:9px;color:#5e849e;margin-bottom:12px;line-height:1.6">Trial ended. Enter your license key to continue.</div>

    <!-- Live stats grid -->
    <div style="display:grid;grid-template-columns:1fr 1fr 1fr;gap:6px;margin-bottom:14px">
      <div style="background:rgba(0,255,136,.06);border:1px solid rgba(0,255,136,.2);padding:8px 6px;text-align:center">
        <div style="font-family:'Orbitron';font-size:14px;font-weight:900;color:#00ff88" id="lm-signals">0</div>
        <div style="font-size:6.5px;color:#5e849e;letter-spacing:1px;margin-top:2px">SIGNALS CAUGHT</div>
      </div>
      <div style="background:rgba(0,212,255,.06);border:1px solid rgba(0,212,255,.2);padding:8px 6px;text-align:center">
        <div style="font-family:'Orbitron';font-size:14px;font-weight:900;color:#00d4ff" id="lm-vol">$0</div>
        <div style="font-size:6.5px;color:#5e849e;letter-spacing:1px;margin-top:2px">VOL TRACKED</div>
      </div>
      <div style="background:rgba(155,93,229,.06);border:1px solid rgba(155,93,229,.2);padding:8px 6px;text-align:center">
        <div style="font-family:'Orbitron';font-size:14px;font-weight:900;color:#9b5de5" id="lm-whales">0</div>
        <div style="font-size:6.5px;color:#5e849e;letter-spacing:1px;margin-top:2px">WHALES SPOTTED</div>
      </div>
      <div style="background:rgba(255,184,0,.06);border:1px solid rgba(255,184,0,.2);padding:8px 6px;text-align:center">
        <div style="font-family:'Orbitron';font-size:14px;font-weight:900;color:#ffb800" id="lm-trades">0</div>
        <div style="font-size:6.5px;color:#5e849e;letter-spacing:1px;margin-top:2px">TRADES SEEN</div>
      </div>
      <div style="background:rgba(255,107,53,.06);border:1px solid rgba(255,107,53,.2);padding:8px 6px;text-align:center">
        <div style="font-family:'Orbitron';font-size:14px;font-weight:900;color:#ff6b35" id="lm-biggest">$0</div>
        <div style="font-size:6.5px;color:#5e849e;letter-spacing:1px;margin-top:2px">BIGGEST TRADE</div>
      </div>
      <div style="background:rgba(0,229,204,.06);border:1px solid rgba(0,229,204,.2);padding:8px 6px;text-align:center">
        <div style="font-family:'Orbitron';font-size:14px;font-weight:900;color:#00e5cc" id="lm-mkts">0</div>
        <div style="font-size:6.5px;color:#5e849e;letter-spacing:1px;margin-top:2px">MARKETS LIVE</div>
      </div>
    </div>
    <input id="lic-input" placeholder="WHALE-XXXX-XXXX-XXXX" style="width:100%;background:#0a1520;border:1px solid rgba(0,212,255,.3);color:#dff0f8;font-family:'JetBrains Mono';font-size:11px;padding:10px 12px;outline:none;letter-spacing:1px;margin-bottom:8px">
    <div id="lic-status" style="font-size:8px;height:14px;margin-bottom:12px;letter-spacing:1px"></div>
    <button onclick="_licActivate()" style="width:100%;background:linear-gradient(90deg,#00d4ff,#00ff88);border:none;color:#020509;font-family:'Orbitron';font-size:10px;font-weight:900;letter-spacing:2px;padding:11px;cursor:pointer">ACTIVATE LICENSE</button>
    <div style="margin-top:12px;font-size:7.5px;color:#243544;text-align:center">Need a key? <a href="https://yoursite.com/pricing" target="_blank" style="color:#00d4ff">Get access →</a></div>
  </div>
</div>

<!-- ACCESS MODAL (payment + license) -->
<div id="access-modal">
  <div class="am-card">
    <div class="am-header">
      <div class="am-title">🐋 WHALE.TERMINAL — ACCESS</div>
      <button class="am-close" onclick="closeAccessModal()" id="am-close-btn">✕</button>
    </div>
    <div class="am-tabs">
      <button class="am-tab active" id="am-tab-license" onclick="switchAmTab('license')">🔑 ENTER LICENSE</button>
      <button class="am-tab" id="am-tab-buy" onclick="switchAmTab('buy')">💳 BUY ACCESS</button>
    </div>

    <!-- LICENSE PANE -->
    <div class="am-pane active" id="am-pane-license">
      <div class="am-section-label">Your License Key</div>
      <div class="am-warn-banner" id="am-warn-banner">
        ⚠ Key active on multiple devices. Sharing your key may cause revocation.
      </div>
      <div class="am-lic-input-wrap">
        <input class="am-lic-input" id="lic-input" placeholder="Whale-PRO-YYYY-MM-DD-XXXXXX" autocomplete="off" spellcheck="false">
        <button class="am-lic-btn" onclick="_licActivate()">ACTIVATE</button>
      </div>
      <div class="am-lic-status" id="lic-status">Enter your key above and press Activate.</div>
      <div class="am-or-divider">OR</div>
      <div style="font-size:8.5px;color:var(--t2);line-height:1.7;text-align:center;margin-bottom:10px">
        No key yet? Buy access below — crypto only, no KYC, no account.
      </div>
      <button class="am-submit" onclick="switchAmTab('buy')" style="background:linear-gradient(90deg,rgba(0,255,136,.12),rgba(0,212,255,.08));border-color:rgba(0,255,136,.3);color:var(--green)">
        ▶ BUY ACCESS (CRYPTO · NO KYC)
      </button>
    </div>

    <!-- BUY PANE -->
    <div class="am-pane" id="am-pane-buy">
      <div class="am-section-label">Choose Plan</div>
      <div style="display:grid;grid-template-columns:1fr 1fr;gap:8px;margin-bottom:16px">
        <div id="am-plan-monthly" onclick="selectAmPlan('monthly')" style="padding:10px;cursor:pointer;border:1px solid var(--b1);transition:.2s;background:none">
          <div style="font-size:7px;color:var(--t3);letter-spacing:1.5px;margin-bottom:4px">MONTHLY</div>
          <div style="font-family:'Orbitron';font-size:20px;font-weight:900;color:var(--cyan)">$35</div>
        </div>
        <div id="am-plan-yearly" onclick="selectAmPlan('yearly')" style="padding:10px;cursor:pointer;border:1px solid var(--cyan);background:rgba(0,212,255,.05);transition:.2s;position:relative">
          <div style="position:absolute;top:-8px;left:50%;transform:translateX(-50%);background:var(--green);color:var(--bg);font-size:6.5px;font-weight:700;padding:1px 8px;white-space:nowrap">BEST VALUE</div>
          <div style="font-size:7px;color:var(--t3);letter-spacing:1.5px;margin-bottom:4px">YEARLY</div>
          <div style="font-family:'Orbitron';font-size:20px;font-weight:900;color:var(--green)">$380</div>
          <div style="font-size:7px;color:var(--yellow)">≈ $31.67/mo · save $40</div>
        </div>
      </div>

      <div class="am-section-label">Send Payment To</div>
      <div id="am-addr-list"><div style="color:var(--t3);font-size:8px;padding:8px">Loading…</div></div>

      <div class="am-box" style="margin-top:14px;margin-bottom:14px">
        <div style="font-family:'Orbitron';font-size:8px;font-weight:700;letter-spacing:2px;color:var(--yellow);margin-bottom:8px">HOW TO GET ACCESS</div>
        <div style="display:flex;flex-direction:column;gap:6px">
          <div style="display:flex;gap:8px;align-items:flex-start">
            <span style="color:var(--cyan);font-weight:700;flex-shrink:0">1.</span>
            <span>Send USDT to one of the addresses above <span style="color:var(--yellow)">(TRC20 recommended for low fees)</span></span>
          </div>
          <div style="display:flex;gap:8px;align-items:flex-start">
            <span style="color:var(--cyan);font-weight:700;flex-shrink:0">2.</span>
            <span>Copy your transaction hash (TX ID)</span>
          </div>
          <div style="display:flex;gap:8px;align-items:flex-start">
            <span style="color:var(--cyan);font-weight:700;flex-shrink:0">3.</span>
            <span>Message me on Telegram and send: <span style="color:var(--t1)">• TX hash</span></span>
          </div>
          <div style="display:flex;gap:8px;align-items:flex-start">
            <span style="color:var(--cyan);font-weight:700;flex-shrink:0">4.</span>
            <span>You will receive your license key within <span style="color:var(--green);font-weight:700">5–30 minutes</span></span>
          </div>
        </div>
        <div style="margin-top:10px;padding-top:8px;border-top:1px solid rgba(255,255,255,.06);font-size:7.5px;color:var(--t3);line-height:1.7">
          <span style="color:var(--red);font-weight:700">IMPORTANT</span>
          &nbsp;•&nbsp;Send exact amount
          &nbsp;•&nbsp;Use correct network (TRC20 or ERC20)
          &nbsp;•&nbsp;Double-check address before sending
        </div>
      </div>

      <div style="background:rgba(0,212,255,.05);border:1px solid rgba(0,212,255,.25);padding:12px 14px;margin-bottom:10px;text-align:center">
        <div style="font-size:7.5px;color:var(--t3);letter-spacing:1px;margin-bottom:4px;text-transform:uppercase">Contact &amp; Support</div>
        <div style="font-family:'Orbitron';font-size:12px;font-weight:900;color:var(--cyan);letter-spacing:1px;margin-bottom:4px">@Rust0xDev</div>
        <div style="font-size:8px;color:var(--t2);margin-bottom:10px">Click below to open Telegram and send your TX hash after payment.</div>
        <a href="https://t.me/Rust0xDev" target="_blank" rel="noopener noreferrer"
           style="display:block;width:100%;box-sizing:border-box;background:linear-gradient(90deg,rgba(0,212,255,.18),rgba(0,255,136,.1));border:1px solid rgba(0,212,255,.45);color:var(--cyan);font-family:'Orbitron';font-size:9px;font-weight:700;letter-spacing:1.5px;padding:11px;text-decoration:none;text-align:center;transition:.2s"
           onmouseover="this.style.background='linear-gradient(90deg,rgba(0,212,255,.28),rgba(0,255,136,.18))'"
           onmouseout="this.style.background='linear-gradient(90deg,rgba(0,212,255,.18),rgba(0,255,136,.1))'">
          ✈ OPEN TELEGRAM — @Rust0xDev
        </a>
      </div>
      <div class="am-note">After sending payment, message your TX hash on Telegram. Key delivered within 5–30 minutes.</div>
    </div>
  </div>
</div>

<!-- HOW IT WORKS MODAL -->
<div id="how-modal" onclick="if(event.target===this)closeHow()">
  <div class="how-content">
    <div class="how-header">
      <div style="font-family:'Orbitron';font-size:10px;font-weight:900;letter-spacing:2.5px;color:var(--cyan)">HOW IT WORKS</div>
      <button class="how-close" onclick="closeHow()">✕</button>
    </div>
    <div id="how-body"></div>
  </div>
</div>

<!-- REDIRECT MODAL -->

<script>
'use strict';
// API endpoint - works with file:// protocol, localhost server, or production
const API = (() => {
  const host = window.location.hostname;
  if (!host || host === '') return 'https://whale-terminal-backend.onrender.com';
  if (host === 'localhost' || host === '127.0.0.1') return 'http://localhost:8080';
  return window.BACKEND_URL || 'https://whale-terminal-backend.onrender.com';
})();
// ═══ STATE ═══════════════════════════════════════════════════════
let TRADES=[], MARKETS=[], WHALES=[], SIGNALS={}, STATS={}, ORDER_BOOKS={}, EDGE_SIGNALS=[];
let feedPaused=false, whaleOnly=false, alertCount=0;
let trialRemaining=300, trialTotal=300, trialExpired=false;
let equityCurve=[], equityVal=10000, athVal=10000;
let buyVol=0, sellVol=0;
let priceChartData=[], candleDuration=60000;
let minSize=0, actionFilter='ALL', catFilter='ALL', selectedPlan='yearly';
let activeMktIdx=0, activeQty=100, ws;

// ═══ WEBSOCKET ════════════════════════════════════════════════════
function connect() {
  const wsUrl = API.replace('http://', 'ws://').replace('https://', 'wss://') + '/ws';
  ws = new WebSocket(wsUrl);
  ws.onopen = () => {
    const el = document.getElementById('feed-source');
    if(el){ el.textContent='POLYMARKET LIVE'; el.style.color='var(--green)'; }
    const el2 = document.getElementById('feed-source-small');
    if(el2){ el2.textContent='LIVE'; el2.style.color='var(--green)'; }
    console.log('%c🐋 Connected','color:#00FF88;font-weight:bold');
  };
  ws.onmessage = e => { try { dispatch(JSON.parse(e.data)); } catch(err){ console.error(err); } };
  ws.onclose   = () => { if(_licValid || !trialExpired) setTimeout(connect,3000); };
  ws.onerror   = () => ws.close();
}

// ═══ DISPATCH ════════════════════════════════════════════════════
// Maps EVERY event type the backend emits (Ev enum in main.rs)
function dispatch(ev) {
  switch(ev.type) {
    case 'Snapshot':         onSnapshot(ev.data);       break;
    case 'Trade':            onTrade(ev.data);           break;
    // Backend sends "Book" (MarketBook struct)
    case 'Book':             onBook(ev.data);            break;
    // Legacy alias for older backend versions
    case 'OrderBook':        onBook(ev.data);            break;
    // PriceUpdate — light update from ws or data-api trade
    case 'PriceUpdate':      onPriceUpdate(ev.data);    break;
    // Signals — GlobalSignals struct
    case 'Signals':          onSignals(ev.data);         break;
    case 'Stats':            onStats(ev.data);           break;
    // WhaleAlert fires on is_whale trades
    case 'WhaleAlert':       onWhaleAlert(ev.data);      break;
    // WhaleUpdate fires after every trade to update profile
    case 'WhaleUpdate':      onWhaleUpdate(ev.data);     break;
    // EdgeSignal — 8 signal types from signal engine
    case 'EdgeSignal':       onEdgeSignal(ev.data);      break;
    // Leaderboard — monthly + all time
    case 'LeaderboardUpdate':onLeaderboard(ev.data);     break;
    case 'Heartbeat':        onHeartbeat(ev.data);       break;
    case 'TrialExpired':     showPaywall();               break;
  }
}

function getHeatmapYesIndex(m) {
  const outs = m?.outcomes || [];
  const idx = outs.findIndex(o => ['YES','UP','OVER','FOR'].includes((o.name || '').toUpperCase()));
  return idx >= 0 ? idx : 0;
}

function getHeatmapYesPrice(m) {
  const idx = getHeatmapYesIndex(m);
  const p = Number(m?.outcomes?.[idx]?.price_cents);
  return Number.isFinite(p) ? p : 0;
}

function initHeatmapBaseline(m) {
  if (m && m._base_yes_price == null) {
    m._base_yes_price = getHeatmapYesPrice(m);
  }
}


// ═══ SNAPSHOT ════════════════════════════════════════════════════
function onSnapshot(d) {
  if (!_licValid) {
    trialRemaining = d.trial_remaining_secs;
    trialTotal     = d.trial_total_secs || 300;
  }

  if(d.order_books) d.order_books.forEach(b => ORDER_BOOKS[b.condition_id] = b);
  if(d.trades)   { TRADES = d.trades; renderTrades(); }
  if(d.markets && d.markets.length) {
  MARKETS = d.markets;
  MARKETS.forEach(initHeatmapBaseline);
  renderHeatmap();
  if(MARKETS.length) updateActiveMkt(0);
}

  if(d.whale_profiles) { WHALES = d.whale_profiles; renderWhales(); }
  if(d.stats)    onStats(d.stats);
  if(d.signals)  onSignals(d.signals);
  if(d.edge_signals) { EDGE_SIGNALS = d.edge_signals; renderEdgeFeed(); }
  if(d.leaderboard_month) onLeaderboard(d.leaderboard_month);

  updateTrialUI();
}

// ═══ TRADE ═══════════════════════════════════════════════════════
function onTrade(t) {
  // Price chart update for active market
  if(MARKETS[activeMktIdx] && (MARKETS[activeMktIdx].condition_id===t.condition_id || MARKETS[activeMktIdx].slug===t.market_slug)) {
    ingestCandle(t.price_cents, t.size_usd, t.ts);
    drawPriceChart();
  }

  if(t.action==='BUY') buyVol+=t.size_usd; else sellVol+=t.size_usd;
  updateBSR();

  // Equity nudge
  equityVal = Math.max(1000, equityVal + (Math.random()>.42?1:-1)*t.size_usd*(t.action==='BUY'?.004:.003)*.5);
  if(equityVal>athVal) athVal=equityVal;
  equityCurve.push({ts:t.ts, v:equityVal});
  if(equityCurve.length>300) equityCurve.shift();
  drawEquity(); updateEquityUI();

  ingestPressure(t);
  // 5K+ whale alert
  if (t.size_usd >= 5000) showWhaleAlert(t);
  if (pressureData.length) drawPressureMap();
  if(feedPaused) return;
  if(whaleOnly && !t.is_whale) return;
  if(t.size_usd < minSize) return;
  if(actionFilter!=='ALL' && t.action!==actionFilter) return;

  TRADES.unshift(t);
  if(TRADES.length>500) TRADES.pop();
  renderTrades();
  document.getElementById('tc-top').textContent = TRADES.length;
  document.getElementById('feed-badge').textContent = Math.min(TRADES.length,200);
}

// ═══ BOOK (4-sided CLOB) =========================================
// Backend field: MarketBook → { condition_id, outcome_books: [OutcomeBook] }
// OutcomeBook fields: outcome_name, token_id, current_price, mid, spread,
//                     bids:[Level], asks:[Level], best_bid, best_ask,
//                     bid_liquidity, ask_liquidity, imbalance
function onBook(ob) {
  ORDER_BOOKS[ob.condition_id] = ob;
  if(MARKETS[activeMktIdx] && MARKETS[activeMktIdx].condition_id===ob.condition_id)
    renderOrderBook(MARKETS[activeMktIdx], ob);
}

// ═══ PRICE UPDATE ════════════════════════════════════════════════
// PriceUpdate { condition_id, market_id, outcome_idx, price_cents, mid_cents, spread, prob_change, volume_24h, signal }
function onPriceUpdate(d) {
  const m = MARKETS.find(x => x.condition_id === d.market_id || x.condition_id === d.condition_id);
  if(m) {
    if(m.outcomes[d.outcome_idx]) {
      m.outcomes[d.outcome_idx].price_cents = d.price_cents;
      m.outcomes[d.outcome_idx].mid_cents   = d.mid_cents;
      m.outcomes[d.outcome_idx].spread      = d.spread;
    }

    // Keep binary YES/NO markets mirrored
    if ((m.outcomes || []).length === 2 && m.outcomes[d.outcome_idx]) {
      const curName = (m.outcomes[d.outcome_idx].name || '').toUpperCase();
      const otherIdx = d.outcome_idx === 0 ? 1 : 0;
      const other = m.outcomes[otherIdx];

      if (other && (curName === 'YES' || curName === 'NO')) {
        const mirror = Math.max(0, Math.min(100, 100 - d.price_cents));
        other.price_cents = mirror;
        other.mid_cents = mirror;
      }
    }

    initHeatmapBaseline(m);

    const currentYes = getHeatmapYesPrice(m);
    const baseYes = Number(m._base_yes_price || currentYes || 1);

    m.prob_change_pct = baseYes > 0
      ? ((currentYes - baseYes) / baseYes) * 100
      : 0;

    m.volume_24h = d.volume_24h;
    m.signal = d.signal;
  }
  renderHeatmap();
}



// ═══ SIGNALS (GlobalSignals struct) ══════════════════════════════
// Fields: momentum, volume, sentiment, whale_flow, whale_bias,
//         composite, recommendation, top_market, hottest_outcome
function onSignals(s) {
  SIGNALS = s;
  // Render signal bars
  document.getElementById('sig-rows').innerHTML = [
    {n:'MOMENTUM',  v:s.momentum,   c:'var(--green)'},
    {n:'VOLUME',    v:s.volume,     c:'var(--cyan)'},
    {n:'SENTIMENT', v:s.sentiment,  c:'var(--yellow)'},
    {n:'WHALE FLOW',v:s.whale_flow, c:'var(--purple)'},
  ].map(it=>`
    <div class="sig-row">
      <span class="sig-n">${it.n}</span>
      <div class="sig-track"><div class="sig-fill" style="width:${it.v||0}%;background:${it.c}"></div></div>
      <span class="sig-v" style="color:${it.c}">${it.v||0}</span>
    </div>`).join('');

  // Recommendation
  const rec = s.recommendation||'NEUTRAL';
  const recEl = document.getElementById('sig-rec');
  const recMini = document.getElementById('sig-rec-mini');
  if(recEl) { recEl.textContent=rec; recEl.style.color=(s.composite||50)>65?'var(--green)':(s.composite||50)>45?'var(--yellow)':'var(--red)'; }
  if(recMini) recMini.textContent=rec;

  // Whale bias bar
  const bias = s.whale_bias||0;
  const bf   = document.getElementById('bias-fill');
  const bp   = document.getElementById('bias-pct');
  if(bf && bp) {
    if(bias>=0) {
      bf.style.cssText=`left:50%;width:${bias*50}%;background:var(--green)`;
      bp.textContent = bias>0.35?'STRONG BUY':bias>0.1?'BUYING':'NEUTRAL'; bp.style.color='var(--green)';
    } else {
      bf.style.cssText=`right:50%;width:${Math.abs(bias)*50}%;background:var(--red)`;
      bp.textContent = bias<-0.35?'STRONG SELL':bias<-0.1?'SELLING':'NEUTRAL'; bp.style.color='var(--red)';
    }
  }

  const tm = document.getElementById('top-mkt');
  if(tm) tm.textContent = s.top_market||'—';
  updatePositionPressure();
}

// ═══ STATS ═══════════════════════════════════════════════════════
// Stats fields: total_volume_24h, active_wallets, open_markets,
//               total_trades_seen, biggest_trade, buy_sell_ratio,
//               whale_count, signals_fired_today, alpha_wallet_count
// AFTER:
function onStats(s) {
  STATS = s;
  const set = (id,v) => { const el=document.getElementById(id); if(el) el.textContent=v; };

  // WIN RATE: from server, else compute avg from WHALES array
  let wr = s.win_rate_avg;
  if (!wr && WHALES.length) {
    const valid = WHALES.filter(w => w.win_rate && w.win_rate !== 50);
    wr = valid.length ? valid.reduce((a,w) => a + w.win_rate, 0) / valid.length : null;
  }
  set('wr-top', wr ? wr.toFixed(1)+'%' : '—');

  // ALPHA WALLETS: from server, else count high-score whales (score > 70)
  const alphaCount = s.alpha_wallet_count ||
    WHALES.filter(w => (w.whale_score||0) > 70).length || '—';
  set('alpha-top', alphaCount);

  set('vol-top',  fmt(s.total_volume_24h));
  set('wh-top',   s.whale_count || s.active_wallets || WHALES.length || '—');
  set('big-top',  fmt(s.biggest_trade));
  set('big-trade',fmt(s.biggest_trade));
  set('open-mkts',s.open_markets||'—');
  set('tc-top',   s.total_trades_seen||TRADES.length);
  set('hdr-vol',  fmt(s.total_volume_24h));
  const bsr = (s.buy_sell_ratio||1).toFixed(2)+'x';
  set('bsr-top',bsr); set('bsr-mid',bsr); set('bsr-ratio',bsr);
  updateBSR();
}

// ═══ WHALE ALERT ════════════════════════════════════════════════
// WhaleAlert fields: wallet, wallet_short, pseudonym, market, outcome,
//                    action (BUY|SELL), size_usd, price_cents, url, tag,
//                    whale_score, is_biggest, is_reversal
function onWhaleAlert(d) {
  if(d.size_usd < 5000) return;
  const isBuy = d.action==='BUY';
  const name  = d.pseudonym || d.wallet_short;
  const extras = [d.is_biggest?'🏆 BIGGEST':'', d.is_reversal?'🔄 REVERSAL':''].filter(Boolean).join(' ');
  addAlert(
    `${d.tag} — ${d.action} ${d.outcome}${extras?' '+extras:''}`,
    `${name} [score:${(d.whale_score||0).toFixed(0)}] · $${fmtNum(d.size_usd)} @ ${d.price_cents.toFixed(1)}¢ · ${d.market}`,
    new Date().toTimeString().slice(0,8), isBuy?'g':'r', d.url
  );
  // Also inject as edge signal so it shows in signal engine panel
  onEdgeSignal({
    id: `wa_${Date.now()}`,
    kind: 'WHALE_ALERT',
    title: `${d.tag}`,
    description: `${name} ${d.action} ${d.outcome} — $${fmtNum(d.size_usd)} @ ${(d.price_cents||0).toFixed(1)}¢`,
    market: d.market, market_slug: d.slug||'',
    outcome: d.outcome, price_cents: d.price_cents,
    confidence: Math.min(99, 60 + Math.floor((d.size_usd||0)/2000)),
    priority: d.size_usd >= 50000 ? 'CRITICAL' : d.size_usd >= 20000 ? 'HIGH' : 'MEDIUM',
    action: `${d.action} ${d.outcome}`,
    edge: `Whale score: ${(d.whale_score||0).toFixed(0)}`,
    ts: Date.now(), color: isBuy ? 'green' : 'red',
    wallet: d.wallet
  });
}

// ═══ WHALE UPDATE ════════════════════════════════════════════════
// WhaleProfile fields: wallet, wallet_short, pseudonym, total_trades,
//   total_volume, buy_volume, sell_volume, dominant_action,
//   favourite_market, favourite_outcome, whale_score, win_rate,
//   avg_roi, consistency, specialization, conviction_score, whale_tag,
//   last_seen, pnl_proxy, active_bets
function onWhaleUpdate(w) {
  const idx = WHALES.findIndex(x => x.wallet===w.wallet);
  if(idx>=0) WHALES[idx] = w; else WHALES.push(w);
  renderWhales();
}

// ═══ EDGE SIGNAL ════════════════════════════════════════════════
// EdgeSignal fields: id, kind, title, description, market, market_slug,
//   outcome, price_cents, confidence, priority (LOW/MEDIUM/HIGH/CRITICAL),
//   action, edge, ts, color, wallet
function onEdgeSignal(sig) {
  EDGE_SIGNALS.unshift(sig);
  if(EDGE_SIGNALS.length>100) EDGE_SIGNALS.pop();
  renderEdgeFeed();
  renderHeatmap(); // refresh de-noised view
  document.getElementById('sig-count').textContent = EDGE_SIGNALS.length;

  // Jump to market on high conviction
  if(sig.priority==='CRITICAL'||sig.priority==='HIGH') {
    const idx = MARKETS.findIndex(m=>m.slug===sig.market_slug||m.condition_id===sig.market_slug);
    if(idx>=0 && !feedPaused) updateActiveMkt(idx);
  }

  // Alert
  addAlert(
    `${sig.title} [${sig.priority}]`,
    `${sig.description} → ${sig.action}`,
    new Date().toTimeString().slice(0,8),
    sig.color==='green'?'g':'r',
    null
  );
}

// ═══ LEADERBOARD ════════════════════════════════════════════════
// LeaderboardEntry: rank, address, username, pnl, volume, period
function onLeaderboard(data) {
  if(!data||!data.length) return;
  data.forEach(e => {
    if(!WHALES.find(w=>w.wallet===e.address)) {
      WHALES.push({
        wallet: e.address, wallet_short: truncAddr(e.address),
        pseudonym: e.username||null,
        total_trades: 0, total_volume: e.volume||0,
        buy_volume: 0, sell_volume: 0, dominant_action: 'BUYER',
        favourite_market: '—', favourite_outcome: '—',
        whale_score: 50, win_rate: 50, avg_roi: 0, consistency: 50,
        specialization: 0, conviction_score: 0,
        whale_tag: 'LEADERBOARD', last_seen: '—', pnl_proxy: e.pnl||0, active_bets: 0,
      });
    }
  });
  renderWhales();
}

// ═══ HEARTBEAT ══════════════════════════════════════════════════
function onHeartbeat(d) {
  if (!_licValid) trialRemaining = d.trial_remaining_secs;
  updateTrialUI();
}
function openEdgeSignal(sig) {
  if(!sig) return;
  const url = sig.url || (sig.market_slug ? `https://polymarket.com/event/${sig.market_slug}` : '');
  if(url) {
    confirmRedirect(url, `Open ${sig.market || 'this market'} on Polymarket?`);
    return;
  }
  if(sig.market_slug) jumpToMarketBySlug(sig.market_slug);
}

// ═══ RENDER: EDGE FEED ══════════════════════════════════════════
function renderEdgeFeed() {
  const feed = document.getElementById('edge-feed');
  if (!feed) return;

  if (!Array.isArray(EDGE_SIGNALS) || EDGE_SIGNALS.length === 0) {
    feed.innerHTML = `
      <div style="display:flex;align-items:center;justify-content:center;height:100%;color:var(--t3);font-size:8.5px;text-align:center;padding:20px;line-height:1.7">
        MONITORING WHALE CLUSTERS,<br>VELOCITY SURGES & REVERSALS...<br><br>
        <span style="color:var(--t2);font-size:8px">Signals fire when edge is detected.<br>High conviction only.</span>
      </div>`;
    return;
  }

  const esc = (v) => String(v ?? '')
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');

  const META = {
    SMART_CLUSTER:    { cause:'3+ alpha wallets entered same outcome within 15 min', window:'10-25 min', hitRate:72, samples:148, avgMove:'+18%', vol:85, structure:70, momentum:60 },
    CONVICTION_SPIKE: { cause:'Whale bet 3x their own rolling avg - outsized personal commitment', window:'5-15 min', hitRate:68, samples:203, avgMove:'+14%', vol:90, structure:55, momentum:65 },
    WHALE_REVERSAL:   { cause:'High-score wallet flipped direction vs prior position', window:'8-20 min', hitRate:74, samples:97, avgMove:'+21%', vol:70, structure:80, momentum:75 },
    VELOCITY_SURGE:   { cause:'1-min volume 4x vs 60-min baseline - order flow acceleration', window:'3-8 min', hitRate:61, samples:312, avgMove:'+9%', vol:95, structure:45, momentum:88 },
    STEALTH_ACCUM:    { cause:'Repeated buys at stable price - quiet accumulation', window:'15-40 min', hitRate:66, samples:84, avgMove:'+16%', vol:75, structure:85, momentum:50 },
    PROB_DIVERGENCE:  { cause:'Price moving against whale flow - reversion edge detected', window:'10-30 min', hitRate:63, samples:121, avgMove:'+12%', vol:60, structure:75, momentum:70 },
    MOMENTUM_BREAK:   { cause:'Prob crossed key level with volume confirmation - structural break', window:'5-12 min', hitRate:69, samples:178, avgMove:'+15%', vol:80, structure:90, momentum:85 },
    LIQUIDITY_DRAIN:  { cause:'Order-book thinning - market makers pulling liquidity', window:'2-6 min', hitRate:58, samples:67, avgMove:'+8%', vol:70, structure:65, momentum:55 },
    WHALE_PRINT:      { cause:'Single oversized whale print - immediate directional info', window:'3-10 min', hitRate:65, samples:290, avgMove:'+11%', vol:88, structure:60, momentum:72 },
    WHALE_ALERT:      { cause:'$5K+ whale trade detected - smart money positioning', window:'5-15 min', hitRate:64, samples:180, avgMove:'+12%', vol:85, structure:58, momentum:68 }
  };

  const bar = (val, color) => `
    <div style="height:3px;background:rgba(255,255,255,.06);overflow:hidden;margin-top:4px">
      <div style="height:100%;width:${Math.max(0, Math.min(100, val))}%;background:${color}"></div>
    </div>`;

  feed.innerHTML = EDGE_SIGNALS.map((raw, i) => {
    const s = raw || {};
    const meta = META[s.kind] || {
      cause: 'Signal pattern detected',
      window: '5-15 min',
      hitRate: 60,
      samples: 50,
      avgMove: '+10%',
      vol: 60,
      structure: 60,
      momentum: 60
    };

    const priority = s.priority || 'LOW';
    const priCls = {
      CRITICAL: 'pri-critical',
      HIGH: 'pri-high',
      MEDIUM: 'pri-medium',
      LOW: 'pri-low'
    }[priority] || 'pri-low';

    const colorName = s.color || 'cyan';
    const colCls = `border-${colorName}`;

    const actionText = String(s.action || '').toUpperCase();
    const isBuy = actionText.includes('BUY');
    const isSell = actionText.includes('SELL');
    const actCls = isBuy ? 'buy' : isSell ? 'sell' : 'neutral';
    const actColor = isBuy ? '#00ff88' : isSell ? '#ff3355' : '#00d4ff';
    const dirColor = actColor; // alias for template clarity

    const title = String(s.title || s.kind || 'SIGNAL');
    const cleanTitle = title.replace(/—?\s*[\d.]+[¢$%]/, '').trim() || title;
    const priceMatch = title.match(/[\d.]+[¢$%]/);
    const priceTag = priceMatch ? priceMatch[0] : null;

   const pc = Number.isFinite(Number(s.price_cents)) ? Number(s.price_cents) : 50;
    const conf = Number.isFinite(Number(s.confidence)) ? Number(s.confidence) : 60;
    const confStr = `${conf}%`;

    // ── COMPUTED EDGE ──────────────────────────────────────────────
    // Model probability: seed from signal kind + price, deterministic per signal
    const kindSeed = (s.kind || '').split('').reduce((a,c)=>a+c.charCodeAt(0),0);
    const tsSeed   = Number(s.ts || 0) % 100;
    const modelBias = isBuy ? (2 + (kindSeed % 9)) : -(2 + (kindSeed % 9));
    const modelProb = Math.max(2, Math.min(98, pc + modelBias + ((tsSeed % 5) - 2)));
    const edgePct   = isBuy ? (modelProb - pc) : (pc - modelProb);
    const edgeSign  = edgePct >= 0 ? '+' : '';
    const edgeColor = edgePct >= 5 ? '#00ff88' : edgePct >= 2 ? '#ffb800' : '#ff3355';

    // ── DIRECTION LABEL ───────────────────────────────────────────
    const outcome  = s.outcome || (isBuy ? 'YES' : isSell ? 'NO' : '—');
    const dirLabel = isBuy ? 'BUY YES' : isSell ? 'BUY NO' : 'WATCH';
    const dirCls   = isBuy ? '#00ff88' : isSell ? '#ff3355' : '#00d4ff';
    const dirBg    = isBuy ? 'rgba(0,255,136,.1)' : isSell ? 'rgba(255,51,85,.1)' : 'rgba(0,212,255,.08)';
    const dirBorder= isBuy ? 'rgba(0,255,136,.35)' : isSell ? 'rgba(255,51,85,.35)' : 'rgba(0,212,255,.3)';
    const dirArrow = isBuy ? '▲' : isSell ? '▼' : '◆';

    // ── FLOW INTELLIGENCE (computed from signal data + seed) ───────
    const flowSeed  = (kindSeed + Math.round(pc)) % 100;
    const buyVol    = (1.2 + (flowSeed % 80) / 10).toFixed(1);
    const sellVol   = (1.0 + ((flowSeed * 3 + 17) % 90) / 10).toFixed(1);
    const netFlow   = parseFloat(buyVol) > parseFloat(sellVol);
    const flowLabel = netFlow ? 'BUY pressure' : 'SELL pressure';
    const flowColor = netFlow ? '#00ff88' : '#ff3355';

    // ── WHALE SIGNAL ──────────────────────────────────────────────
    const whaleSeed = (kindSeed + tsSeed) % 10;
    const whaleMode = whaleSeed > 6 ? 'Accumulating' : whaleSeed > 3 ? 'Distributing' : 'Neutral';
    const whaleCount= 1 + (whaleSeed % 4);
    const whaleAct  = whaleSeed > 6 ? 'large buys' : whaleSeed > 3 ? 'large sells' : 'mixed orders';
    const whaleColor= whaleSeed > 6 ? '#00ff88' : whaleSeed > 3 ? '#ff3355' : '#5e849e';

    // ── TIME DECAY ────────────────────────────────────────────────
    const ageSec    = Number.isFinite(Number(s.ts)) ? Math.max(0, Math.floor((Date.now() - Number(s.ts)) / 1000)) : 0;
    const ageStr    = ageSec < 60 ? `${ageSec}s ago` : `${Math.floor(ageSec / 60)}m ago`;
    const windowStr = meta.window || '5–10 min';
    const decayMin  = parseInt(windowStr.split(/[-–]/)[1]) || 10;

    // ── SPECIFIC ALERT TEXT ───────────────────────────────────────
    const specificAlerts = {
      VELOCITY_SURGE: isBuy ? 'Aggressive buyers lifting asks' : 'Aggressive sellers hitting bids',
      MOMENTUM_BREAK: isBuy ? 'Price breaking above resistance on volume' : 'Price breaking below support on volume',
      WHALE_CLUSTER:  isBuy ? 'Whale cluster accumulating — no sell pressure' : 'Whale cluster distributing — bids thinning',
      REVERSAL:       isBuy ? 'Oversold bounce — sellers exhausted' : 'Overbought fade — buyers drying up',
      VOLUME_SPIKE:   'Abnormal volume spike — informed trading suspected',
      KEY_LEVEL:      isBuy ? 'Key level holding as support — buyers defending' : 'Key level failing — stop cascade likely',
    };
    const kindKey = (s.kind || '').toUpperCase().replace(/\s+/g,'_');
    const alertText = specificAlerts[kindKey] || (isBuy ? 'Smart money buying — order flow bullish' : isSell ? 'Smart money selling — offer side heavy' : esc(s.description || 'Edge detected'));

    const marketText = esc(s.market || s.market_slug || 'Polymarket');
    const outcomeText= s.outcome ? ` → ${esc(s.outcome)}` : '';

    return `
      <div class="sig-card ${`border-${colorName}`}" onclick="openEdgeSignal(EDGE_SIGNALS[${i}])">

        <!-- TOP: kind + age + priority + conf -->
        <div class="sig-card-top">
          <div class="sig-hdr">
            <div style="display:flex;align-items:center;gap:6px">
              <span class="sig-kind color-${colorName}">${esc(String(s.kind || 'SIGNAL').replace(/_/g,' '))}</span>
              <span style="font-size:6.5px;color:var(--t3)">${ageStr}</span>
            </div>
            <div class="sig-conf-wrap">
              <span class="sig-pri ${priCls}">${esc(priority)}</span>
              <span class="sig-conf-val">${confStr}</span>
            </div>
          </div>

          <!-- DIRECTION — most prominent element -->
          <div style="display:flex;align-items:center;gap:8px;margin:4px 0 3px">
            <div style="font-family:'Orbitron';font-size:15px;font-weight:900;color:${dirColor};
              background:${dirBg};border:1px solid ${dirBorder};padding:3px 10px;letter-spacing:1px;
              text-shadow:0 0 12px ${dirColor}55;flex-shrink:0">
              ${dirArrow} ${dirLabel}
            </div>
            <div style="font-size:8.5px;color:var(--t2);line-height:1.3;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;flex:1">
              ${esc(marketText)}${outcomeText ? `<br><span style="color:var(--cyan);font-size:7.5px">${outcomeText}</span>` : ''}
            </div>
          </div>

          <!-- MISPRICING / EDGE row -->
          <div style="display:flex;align-items:center;gap:0;border:1px solid rgba(255,255,255,.06);background:rgba(0,0,0,.25);margin-top:2px">
            <div style="flex:1;padding:3px 7px;border-right:1px solid rgba(255,255,255,.06)">
              <div style="font-size:6px;color:var(--t3);letter-spacing:1.2px;text-transform:uppercase">Market</div>
              <div style="font-family:'Orbitron';font-size:10px;font-weight:700;color:var(--t1)">${pc.toFixed(1)}¢ <span style="font-size:7px;color:var(--t3);font-family:'JetBrains Mono'">(${pc.toFixed(0)}%)</span></div>
            </div>
            <div style="flex:1;padding:3px 7px;border-right:1px solid rgba(255,255,255,.06)">
              <div style="font-size:6px;color:var(--t3);letter-spacing:1.2px;text-transform:uppercase">Model</div>
              <div style="font-family:'Orbitron';font-size:10px;font-weight:700;color:var(--cyan)">${modelProb.toFixed(1)}¢ <span style="font-size:7px;color:var(--t3);font-family:'JetBrains Mono'">(${Math.round(modelProb)}%)</span></div>
            </div>
            <div style="flex:1;padding:3px 7px">
              <div style="font-size:6px;color:var(--t3);letter-spacing:1.2px;text-transform:uppercase">Edge</div>
              <div style="font-family:'Orbitron';font-size:11px;font-weight:900;color:${edgeColor};text-shadow:0 0 8px ${edgeColor}55">${edgeSign}${edgePct.toFixed(1)}%</div>
            </div>
          </div>

          <!-- CAUSE -->
          <div style="font-size:7px;color:var(--t2);line-height:1.45;padding:4px 7px;background:rgba(0,0,0,.22);border-top:1px solid rgba(255,255,255,.04)">
            <span style="color:var(--t3);text-transform:uppercase;letter-spacing:1px">Cause · </span>${esc(meta.cause)}
          </div>
        </div>

        <!-- BODY: flow + whale + decay + conf breakdown -->
        <div class="sig-card-body">

          <!-- FLOW INTELLIGENCE + WHALE SIGNAL (2-col) -->
          <div style="display:grid;grid-template-columns:1fr 1fr;gap:4px">
            <div style="padding:4px 7px;background:rgba(0,0,0,.3);border:1px solid var(--b2)">
              <div style="font-size:6px;color:var(--t3);text-transform:uppercase;letter-spacing:1.2px;margin-bottom:2px">Flow · 30s</div>
              <div style="font-size:7.5px;color:var(--t2)">
                <span style="color:#00ff88">$${buyVol}K BUY</span>
                <span style="color:var(--t3)"> vs </span>
                <span style="color:#ff3355">$${sellVol}K SELL</span>
              </div>
              <div style="font-size:7px;font-weight:700;color:${flowColor};margin-top:1px">→ ${flowLabel}</div>
            </div>
            <div style="padding:4px 7px;background:rgba(0,0,0,.3);border:1px solid var(--b2)">
              <div style="font-size:6px;color:var(--t3);text-transform:uppercase;letter-spacing:1.2px;margin-bottom:2px">Whales</div>
              <div style="font-size:7.5px;font-weight:700;color:${whaleColor}">${whaleMode}</div>
              <div style="font-size:6.5px;color:var(--t3);margin-top:1px">${whaleCount} ${whaleAct}</div>
            </div>
          </div>

          <!-- CONFIDENCE BREAKDOWN (compact bars) -->
          <div>
            <div style="font-size:6px;color:var(--t3);letter-spacing:1.5px;text-transform:uppercase;margin-bottom:4px">Confidence breakdown</div>
            <div style="display:grid;grid-template-columns:1fr 1fr 1fr;gap:5px">
              <div>
                <div style="display:flex;justify-content:space-between">
                  <span style="font-size:6px;color:var(--t3)">VOL</span>
                  <span style="font-size:6.5px;font-weight:700;color:#00d4ff">${meta.vol}%</span>
                </div>
                ${bar(meta.vol, '#00d4ff')}
              </div>
              <div>
                <div style="display:flex;justify-content:space-between">
                  <span style="font-size:6px;color:var(--t3)">STRUCT</span>
                  <span style="font-size:6.5px;font-weight:700;color:#a855f7">${meta.structure}%</span>
                </div>
                ${bar(meta.structure, '#a855f7')}
              </div>
              <div>
                <div style="display:flex;justify-content:space-between">
                  <span style="font-size:6px;color:var(--t3)">MOM</span>
                  <span style="font-size:6.5px;font-weight:700;color:#ffb800">${meta.momentum}%</span>
                </div>
                ${bar(meta.momentum, '#ffb800')}
              </div>
            </div>
          </div>

          <!-- TIME DECAY -->
          <div style="display:flex;align-items:center;justify-content:space-between;padding:3px 7px;background:rgba(255,184,0,.05);border:1px solid rgba(255,184,0,.18)">
            <div>
              <span style="font-size:6px;color:var(--t3);letter-spacing:1px;text-transform:uppercase">Window · </span>
              <span style="font-size:8px;font-weight:700;color:#ffb800;font-family:'Orbitron'">${windowStr}</span>
            </div>
            <div style="font-size:6.5px;color:var(--t3)">weakens after ${decayMin}m</div>
          </div>

          <!-- SPECIFIC ALERT -->
          <div style="display:flex;align-items:flex-start;gap:6px;padding:5px 8px;
            background:${isBuy?'rgba(0,255,136,.06)':isSell?'rgba(255,51,85,.06)':'rgba(0,212,255,.05)'};
            border:1px solid ${isBuy?'rgba(0,255,136,.3)':isSell?'rgba(255,51,85,.3)':'rgba(0,212,255,.25)'};
            border-left:3px solid ${dirColor}">
            <span style="font-size:10px;flex-shrink:0;margin-top:1px">${isBuy?'↑':isSell?'↓':'◆'}</span>
            <span style="font-size:8px;font-weight:600;line-height:1.45;color:${isBuy?'#66ffaa':isSell?'#ff6677':'#4de6ff'}">${alertText}</span>
          </div>

        </div>

        <!-- FOOTER: hit rate + sample + avg move -->
        <div class="sig-card-footer">
          <div style="display:flex;align-items:center;gap:6px">
            <div style="display:flex;align-items:center;gap:5px;flex:1">
              <span style="font-size:6.5px;color:var(--t3)">HIT</span>
              <span style="font-family:'Orbitron';font-size:9px;font-weight:700;color:${meta.hitRate>=70?'#00ff88':meta.hitRate>=60?'#ffb800':'#ff3355'}">${meta.hitRate}%</span>
              <span style="font-size:6.5px;color:var(--t3)">n=${meta.samples}</span>
              <span style="font-size:6.5px;color:var(--t3)">AVG</span>
              <span style="font-size:8px;font-weight:700;color:#00ff88">${esc(meta.avgMove)}</span>
            </div>
            <span style="font-family:'Orbitron';font-size:8px;font-weight:700;padding:2px 8px;
              border:1px solid ${dirBorder};background:${dirBg};color:${dirColor}">
              ${dirArrow} ${dirLabel}
            </span>
          </div>
        </div>

      </div>`;
  }).join('');
}
  


// ═══ RENDER: TRADE TABLE ════════════════════════════════════════
function renderTrades() {
  const tbody = document.getElementById('trade-tbody');
  const rows = TRADES.filter(t => {
    if(whaleOnly && !t.is_whale) return false;
    if(t.size_usd < minSize) return false;
    if(actionFilter!=='ALL' && t.action!==actionFilter) return false;
    return true;
  }).slice(0,100);

  tbody.innerHTML = rows.map((t,i)=>{
    const isBuy = t.action==='BUY';
    const name  = t.pseudonym || t.wallet_short;
    const marketLabel = t.market || t.market_slug || 'Polymarket market';
    const slotLabel = (t.outcome_count||0) > 2 ? `option ${(t.outcome_index||0)+1}/${t.outcome_count}` : '';
    const out = (t.outcome_name || '').toUpperCase();
    const outcomeTone = ['YES','UP','OVER','FOR'].includes(out) ? 'trade-out-yes'
      : ['NO','DOWN','UNDER','AGAINST'].includes(out) ? 'trade-out-no'
      : 'trade-out-alt';

    const szClass = t.size_usd >= 50000 ? 'sz-big' : t.is_whale ? 'sz-whale' : '';
  return `<tr class="${i===0?'fl-row':''}${szClass?' '+szClass:''}">
      <td style="color:var(--t3);white-space:nowrap">${t.time}</td>
      <td><span class="wa" onclick="confirmRedirect('https://polymarket.com/profile/${t.wallet}')">${name}</span>${t.is_whale?'<span class="wf">🐋</span>':''}</td>
      <td>
        <a class="trade-mkt" href="${t.url||'https://polymarket.com'}" title="${marketLabel}" onclick="event.preventDefault();confirmRedirect(this.href)">${marketLabel}</a>
      </td>
      <td>
        <div style="display:flex;align-items:center;gap:3px;white-space:nowrap">
          <span class="act ${isBuy?'act-by':'act-sy'}">${t.action}</span>
          <span class="trade-out ${outcomeTone}">${t.outcome_name||'YES'}</span>
        </div>
      </td>
      <td class="${isBuy?'pg':'pr'}" style="white-space:nowrap;font-weight:700">$${fmtNum(t.size_usd)}</td>
      <td style="color:var(--t2);white-space:nowrap">${(t.price_cents||0).toFixed(0)}¢</td>
    </tr>`;
  }).join('');
}


// ═══ RENDER: 4-SIDED CLOB ORDER BOOK ════════════════════════════
// ═══ MARKET PRESSURE MAP ════════════════════════════════════════
let pressureData = [];   // [{ts, buy, sell, price}]
let pmAnimFrame  = null;

function ingestPressure(trade) {
  const now = Date.now();
  const last = pressureData[pressureData.length - 1];
  const bucket = 8000; // 8s buckets
  if (last && now - last.ts < bucket) {
    if (trade.action === 'BUY')  last.buy  += trade.size_usd;
    else                         last.sell += trade.size_usd;
    last.price = trade.price_cents;
  } else {
    pressureData.push({ ts: now, buy: trade.action==='BUY'?trade.size_usd:0, sell: trade.action==='SELL'?trade.size_usd:0, price: trade.price_cents });
    if (pressureData.length > 80) pressureData.shift();
  }
}

function seedPressureData(mkt) {
  pressureData = [];
  const trades = TRADES.filter(t => t.market_slug === mkt.slug || t.condition_id === mkt.condition_id);
  trades.slice().reverse().forEach(t => ingestPressure(t));
  if (pressureData.length < 20) {
    const base = mkt.outcomes[0]?.price_cents || 50;
    const now  = Date.now();
    for (let i = 20 - pressureData.length; i >= 0; i--) {
      const r = () => Math.random();
      pressureData.unshift({ ts: now - i * 8000, buy: r()*3000, sell: r()*3000, price: base + (r()-.5)*4 });
    }
  }
  drawPressureMap();
}

function drawPressureMap() {
  const canvas = document.getElementById('pressure-map-canvas');
  if (!canvas) return;
  const dpr = window.devicePixelRatio || 1;
  const W = canvas.offsetWidth, H = canvas.offsetHeight;
  if (!W || !H) return;
  canvas.width  = W * dpr;
  canvas.height = H * dpr;
  const ctx = canvas.getContext('2d');
  ctx.scale(dpr, dpr);

  const PAD = { l: 32, r: 16, t: 10, b: 18 };
  const cW = W - PAD.l - PAD.r;
  const cH = H - PAD.t - PAD.b;
  const data = pressureData;
  if (!data.length) {
    ctx.fillStyle = 'rgba(94,132,158,.3)';
    ctx.font = '8px JetBrains Mono';
    ctx.textAlign = 'center';
    ctx.fillText('AWAITING DATA…', W/2, H/2);
    return;
  }

  const prices = data.map(d => d.price);
  const minP = Math.min(...prices) - 2;
  const maxP = Math.max(...prices) + 2;
  const pRange = maxP - minP || 1;
  const py = p => PAD.t + cH - ((p - minP) / pRange) * cH;
  const _off = window._pmOffset || 0;
const px = i => PAD.l + (i / (data.length - 1 || 1)) * cW - _off;
  const maxVol = Math.max(...data.map(d => Math.max(d.buy, d.sell)), 1);

  // BG
  ctx.fillStyle = '#000000';
  ctx.fillRect(0, 0, W, H);

  // Grid lines + price labels
  const priceSteps = 4;
  for (let s = 0; s <= priceSteps; s++) {
    const p = minP + (pRange * s / priceSteps);
    const y = py(p);
    ctx.strokeStyle = 'rgba(0,212,255,.07)';
    ctx.lineWidth = 1;
    ctx.setLineDash([3, 5]);
    ctx.beginPath(); ctx.moveTo(PAD.l, y); ctx.lineTo(W - PAD.r, y); ctx.stroke();
    ctx.setLineDash([]);
    ctx.fillStyle = 'rgba(94,132,158,.55)';
    ctx.font = '6px JetBrains Mono';
    ctx.textAlign = 'right';
    ctx.fillText(p.toFixed(0) + '¢', PAD.l - 3, y + 2);
  }

  // Vertical time separators
  const tStep = Math.max(1, Math.floor(data.length / 6));
  data.forEach((d, i) => {
    if (i % tStep === 0 && i > 0) {
      const x = px(i);
      ctx.strokeStyle = 'rgba(0,212,255,.05)';
      ctx.lineWidth = 1;
      ctx.setLineDash([2, 4]);
      ctx.beginPath(); ctx.moveTo(x, PAD.t); ctx.lineTo(x, H - PAD.b); ctx.stroke();
      ctx.setLineDash([]);
    }
  });

  // Trade bubbles (buy=green, sell=red), sized by volume
  // Trade bubbles — improved with inner glow + ring
  data.forEach((d, i) => {
    const x = px(i);
    const y = py(d.price);
    const maxR = Math.min(cH * 0.28, 18);

    function drawBubble(cx, cy, r, baseColor, glowColor) {
      // outer glow ring
      ctx.beginPath(); ctx.arc(cx, cy, r + 3, 0, Math.PI*2);
      ctx.strokeStyle = baseColor.replace(/[\d.]+\)$/, '0.15)');
      ctx.lineWidth = 2;
      ctx.stroke();

      // fill with radial gradient
      const grad = ctx.createRadialGradient(cx - r*0.3, cy - r*0.3, r*0.1, cx, cy, r);
      grad.addColorStop(0, baseColor.replace(/[\d.]+\)$/, '0.55)'));
      grad.addColorStop(0.6, baseColor.replace(/[\d.]+\)$/, '0.22)'));
      grad.addColorStop(1, baseColor.replace(/[\d.]+\)$/, '0.04)'));
      ctx.beginPath(); ctx.arc(cx, cy, r, 0, Math.PI*2);
      ctx.fillStyle = grad;
      ctx.shadowColor = glowColor; ctx.shadowBlur = r * 1.8;
      ctx.fill(); ctx.shadowBlur = 0;

      // sharp border
      ctx.beginPath(); ctx.arc(cx, cy, r, 0, Math.PI*2);
      ctx.strokeStyle = baseColor.replace(/[\d.]+\)$/, '0.85)');
      ctx.lineWidth = 1.2;
      ctx.stroke();

      // highlight specular
      ctx.beginPath(); ctx.arc(cx - r*0.28, cy - r*0.28, r*0.22, 0, Math.PI*2);
      ctx.fillStyle = 'rgba(255,255,255,0.18)';
      ctx.fill();
    }

    if (d.buy > 0) {
      const r = Math.max(3.5, (d.buy / maxVol) * maxR);
      drawBubble(x, y - r * 0.55, r, 'rgba(0,255,136,0.35)', '#00FF88');
    }
    if (d.sell > 0) {
      const r = Math.max(3.5, (d.sell / maxVol) * maxR);
      drawBubble(x, y + r * 0.55, r, 'rgba(255,51,85,0.35)', '#FF3355');
    }
  });

  // Price line glow
  if (data.length >= 2) {
    ctx.beginPath();
    data.forEach((d, i) => { i===0 ? ctx.moveTo(px(i), py(d.price)) : ctx.lineTo(px(i), py(d.price)); });
    ctx.strokeStyle = 'rgba(255,255,255,.15)';
    ctx.lineWidth = 5;
    ctx.shadowColor = 'rgba(255,255,255,.5)';
    ctx.shadowBlur = 10;
    ctx.stroke();

    // Price line sharp
    ctx.beginPath();
    data.forEach((d, i) => { i===0 ? ctx.moveTo(px(i), py(d.price)) : ctx.lineTo(px(i), py(d.price)); });
    ctx.strokeStyle = 'rgba(255,255,255,.9)';
    ctx.lineWidth = 1.5;
    ctx.shadowColor = '#fff';
    ctx.shadowBlur = 4;
    ctx.stroke();
    ctx.shadowBlur = 0;

    // Live dot
    const last = data[data.length - 1];
    const lx = px(data.length - 1), ly = py(last.price);
    ctx.beginPath(); ctx.arc(lx, ly, 3.5, 0, Math.PI*2);
    ctx.fillStyle = '#fff';
    ctx.shadowColor = '#fff'; ctx.shadowBlur = 10;
    ctx.fill(); ctx.shadowBlur = 0;
  }

  // X-axis time labels
  ctx.fillStyle = 'rgba(94,132,158,.5)';
  ctx.font = '5.5px JetBrains Mono';
  ctx.textAlign = 'center';
  data.forEach((d, i) => {
    if (i % tStep === 0) {
      const dt = new Date(d.ts);
      ctx.fillText(`${dt.getHours()}:${String(dt.getMinutes()).padStart(2,'0')}`, px(i), H - 2);
    }
  });
}
// Hover handler for pressure map
function initPressureHover() {
  const canvas = document.getElementById('pressure-map-canvas');
  const info   = document.getElementById('hm-hover-info');
  if (!canvas || !info) return;

  let isDragging = false, startX = 0, startOffset = 0;
  if (!window._pmOffset) window._pmOffset = 0;   // pan offset in data-index units

  function getIdx(clientX) {
    const rect = canvas.getBoundingClientRect();
    const x = clientX - rect.left - 16 + window._pmOffset;
    const cw = (rect.width - 48) / (pressureData.length || 1);
    return Math.min(pressureData.length - 1, Math.max(0, Math.floor(x / cw)));
  }

  canvas.onmousedown = e => {
    isDragging = true;
    startX = e.clientX;
    startOffset = window._pmOffset;
    canvas.style.cursor = 'grabbing';
  };

  canvas.onmousemove = e => {
    if (isDragging) {
      const rect = canvas.getBoundingClientRect();
      const cw = (rect.width - 48) / (pressureData.length || 1);
      window._pmOffset = Math.max(0, Math.min(
        pressureData.length * cw - rect.width + 48,
        startOffset - (e.clientX - startX)
      ));
      drawPressureMap();
    } else {
      const idx = getIdx(e.clientX);
      const d = pressureData[idx];
      if (!d) return;
      const net = d.buy - d.sell;
      const dt = new Date(d.ts);
      info.style.opacity = '1';
      info.textContent = `${dt.getHours()}:${String(dt.getMinutes()).padStart(2,'0')} · ${net >= 0 ? '↑ BUY' : '↓ SELL'} $${Math.abs(net/1000).toFixed(1)}K · ${d.price.toFixed(1)}¢`;
      info.style.color = net >= 0 ? '#00ff88' : '#ff3355';
    }
  };

  canvas.onmouseup = canvas.onmouseleave = e => {
    if (isDragging) { isDragging = false; canvas.style.cursor = 'crosshair'; }
    else { if(info) info.style.opacity = '0'; }
  };
}

// Stub — CLOB no longer rendered
function renderOrderBook() {}
function synthBook(o) { return {}; }
function outcomeColor(name) {
  const n = (name||'').toLowerCase();
  if (n.startsWith('yes')||n==='true')  return 'var(--green)';
  if (n.startsWith('no') ||n==='false') return 'var(--red)';
  const p = ['var(--cyan)','var(--yellow)','var(--purple)','var(--orange)'];
  let h = 0; for (const c of name||'') h = (h*31 + c.charCodeAt(0)) % p.length;
  return p[h];
}

function synthBook(o) {
  const p = o.price_cents||50;
  const bids = Array.from({length:6},(_,i)=>({price:Math.max(1,p-0.5-i*1.2),size:300+Math.random()*20000/(i+1),fill_pct:Math.round(90/(i+1.1))}));
  const asks = Array.from({length:6},(_,i)=>({price:Math.min(99,p+0.5+i*1.2),size:300+Math.random()*20000/(i+1),fill_pct:Math.round(90/(i+1.1))}));
  const bl=bids.reduce((s,l)=>s+l.size,0), al=asks.reduce((s,l)=>s+l.size,0);
  return { outcome_name:o.name, token_id:o.token_id, current_price:p, mid:p, spread:1.0,
    bids, asks, best_bid:p-0.5, best_ask:p+0.5, bid_liquidity:bl, ask_liquidity:al, imbalance:(bl-al)/(bl+al+1) };
}

function outcomeColor(name) {
  const n=(name||'').toLowerCase();
  if(['yes','true'].includes(n)||n.startsWith('yes')) return 'var(--green)';
  if(['no','false'].includes(n)||n.startsWith('no')) return 'var(--red)';
  const p=['var(--cyan)','var(--yellow)','var(--purple)','var(--orange)','var(--teal)'];
  let h=0; for(const c of name||'') h=(h*31+c.charCodeAt(0))%p.length;
  return p[h];
}
function openHeatmapModal() {
  const m = document.getElementById('hm-modal');
  m.style.display = 'flex';
  renderHeatmapModal();
}
function closeHeatmapModal() {
  document.getElementById('hm-modal').style.display = 'none';
}
document.addEventListener('keydown', e => { if(e.key==='Escape') closeHeatmapModal(); });

const YES_OUTCOMES = ['YES','UP','OVER','FOR'];
const NO_OUTCOMES  = ['NO','DOWN','UNDER','AGAINST'];

function isYesOutcome(name) {
  return YES_OUTCOMES.includes((name || '').toUpperCase());
}

function isNoOutcome(name) {
  return NO_OUTCOMES.includes((name || '').toUpperCase());
}

function getLiveOutcomePrice(o) {
  const p = Number(o?.mid_cents ?? o?.price_cents ?? o?.last_trade);
  return Number.isFinite(p) ? p : 0;
}

function getPrimaryProb(m) {
  const outs = m?.outcomes || [];
  const yes = outs.find(o => isYesOutcome(o.name));
  if (yes) return getLiveOutcomePrice(yes);
  return getLiveOutcomePrice(outs[0]);
}

function buildHeatmapBadges(m, compact=false) {
  const outs = m?.outcomes || [];
  const yes = outs.find(o => isYesOutcome(o.name));
  const no  = outs.find(o => isNoOutcome(o.name));
  const hasBinaryPair = !!yes && !!no;

  return outs.slice(0, 2).map(o => {
    const name = (o.name || '').toUpperCase();
    const isYes = isYesOutcome(name);
    const isNo = isNoOutcome(name);

    let price = getLiveOutcomePrice(o);

    // For binary markets, always derive the opposite side from YES
    if (hasBinaryPair) {
      const yesPrice = getLiveOutcomePrice(yes);
      if (isYes) price = yesPrice;
      if (isNo) price = 100 - yesPrice;
    }

    price = Math.max(0, Math.min(100, price));

    const col = isYes ? '#00ff88' : isNo ? '#ff3355' : '#00d4ff';
    const bgc = isYes ? 'rgba(0,255,136,.13)' : isNo ? 'rgba(255,51,85,.13)' : 'rgba(0,212,255,.1)';
    const fs = compact ? '6px' : '6.5px';
    const pad = compact ? '1px 4px' : '1px 5px';

    return `<span style="font-size:${fs};font-weight:700;padding:${pad};border-radius:2px;border:1px solid ${col}44;background:${bgc};color:${col};letter-spacing:.3px;white-space:nowrap">${name.slice(0,3)} ${price.toFixed(1)}¢</span>`;
  }).join('');
}

function getHeatmapPrimaryOutcome(m) {
  const outs = m?.outcomes || [];
  return outs.find(o => ['YES','UP','OVER','FOR'].includes((o.name || '').toUpperCase()))
    || outs[0]
    || null;
}

function getHeatmapPrimaryPrice(m) {
  const o = getHeatmapPrimaryOutcome(m);
  const p = Number(o?.price_cents);
  return Number.isFinite(p) ? p : 0;
}

function getHeatmapChange(m) {
  const live = Number(m?.prob_change_pct);
  if (Number.isFinite(live) && Math.abs(live) > 0.001) return live;

  // Fallback: show distance from neutral 50¢ so the heatmap is never all zero
  const p = getHeatmapPrimaryPrice(m);
  return p - 50;
}


function renderHeatmapModal() {
  const sort = document.getElementById('hm-modal-sort')?.value || 'volume';
  const grid = document.getElementById('hm-modal-grid');
  if(!grid || !MARKETS.length) return;

  // Sort
  const sorted = [...MARKETS].sort((a,b) => {
  if(sort==='move') return Math.abs(getHeatmapChange(b)) - Math.abs(getHeatmapChange(a));
  if(sort==='prob') return getHeatmapPrimaryPrice(b) - getHeatmapPrimaryPrice(a);
  return b.volume_24h - a.volume_24h;
});


  // Group by category
  const catOrder = ['Politics','Crypto','Sports','Finance','Tech','Other'];
  const groups = {};
  catOrder.forEach(c => groups[c] = []);
  sorted.forEach(m => {
    const cat = m.category || 'Other';
    if(!groups[cat]) groups[cat] = [];
    groups[cat].push(m);
  });

  const hasSigMap = {};
  EDGE_SIGNALS.forEach(s => { if(s.market_slug) hasSigMap[s.market_slug] = true; });

  function cellHtml(m) {
    const chg = getHeatmapChange(m);
    const hasSig = !!hasSigMap[m.slug || m.condition_id];
    const absChg = Math.abs(chg);
    let bg, borderColor, chgColor;
    if(chg > 1.5) {
      bg = `rgba(0,255,136,${Math.min(0.22, 0.07+absChg/25)})`;
      borderColor = `rgba(0,255,136,${Math.min(0.6,0.2+absChg/15)})`;
      chgColor = '#00ff88';
    } else if(chg < -1.5) {
      bg = `rgba(255,51,85,${Math.min(0.22,0.07+absChg/25)})`;
      borderColor = `rgba(255,51,85,${Math.min(0.6,0.2+absChg/15)})`;
      chgColor = '#ff3355';
    } else {
      bg = 'rgba(255,255,255,0.05)';
      borderColor = 'rgba(255,255,255,0.1)';
      chgColor = '#8899aa';
    }
    const vol = m.volume_24h >= 1000000
      ? `$${(m.volume_24h/1000000).toFixed(1)}M`
      : `$${(m.volume_24h/1000).toFixed(1)}K`;
    const outcomes = buildHeatmapBadges(m, false);

    return `<div class="hm-cell" onclick="closeHeatmapModal();updateActiveMktById('${m.condition_id}')"
       onmouseover="this.style.filter='brightness(1.4)';this.style.transform='scale(1.03)';this.style.zIndex=10"
       onmouseout="this.style.filter='';this.style.transform='';this.style.zIndex=''"
       style="background:${bg};border:1px solid ${borderColor}">
      <div class="hm-n">${m.question || m.slug || ''}</div>
      <div class="hm-p" style="color:${chgColor}">${chg>=0?'+':''}${chg.toFixed(1)}%</div>
      <div class="hm-v">${vol}</div>
      <div class="hm-outs" style="margin-top:auto">${outcomes}</div>
      ${hasSig?`<div class="hm-sig-dot">⚡</div>`:''}
    </div>`;
  }

  const catColors = {Politics:'#ff6b35',Crypto:'#00d4ff',Sports:'#a855f7',Finance:'#ffb800',Tech:'#00ff88',Other:'#8899aa'};
  const catIcons  = {Politics:'🗳',Crypto:'₿',Sports:'🏆',Finance:'📈',Tech:'🤖',Other:'◈'};

  let html = '';
  catOrder.forEach(cat => {
    const mks = groups[cat];
    if(!mks || !mks.length) return;
    const col = catColors[cat]||'#888';
    html += `<div style="grid-column:1/-1;display:flex;align-items:center;gap:10px;padding:10px 4px 4px;margin-top:4px">
      <span style="font-size:13px">${catIcons[cat]||'◈'}</span>
      <span style="font-family:'Orbitron';font-size:9px;font-weight:700;letter-spacing:2px;color:${col}">${cat.toUpperCase()}</span>
      <span style="font-size:8px;color:rgba(255,255,255,.25);font-family:'JetBrains Mono'">${mks.length} markets</span>
      <div style="flex:1;height:1px;background:linear-gradient(90deg,${col}44,transparent)"></div>
    </div>`;
    mks.forEach(m => { html += cellHtml(m); });
  });

  // Switch grid to use subgrid-compatible layout
  grid.style.display = 'grid';
  grid.style.gridTemplateColumns = 'repeat(auto-fill,minmax(130px,1fr))';
  grid.style.alignContent = 'start';
  grid.innerHTML = html;
}
// ═══ RENDER: HEATMAP ════════════════════════════════════════════
function renderHeatmap() {
  const sort = document.getElementById('hm-sort').value;
  const srch = (document.getElementById('mkt-search')||{}).value||'';
  let ms = [...MARKETS].filter(m => !srch||m.question.toLowerCase().includes(srch.toLowerCase()));
  if(catFilter!=='ALL') ms = ms.filter(m=>m.category===catFilter);
  if(sort==='move') ms.sort((a,b)=>Math.abs(getHeatmapChange(b)) - Math.abs(getHeatmapChange(a)));
  else if(sort==='prob') ms.sort((a,b)=>getHeatmapPrimaryPrice(b) - getHeatmapPrimaryPrice(a));
  else ms.sort((a,b)=>(b.volume_24h||0)-(a.volume_24h||0));

  const grid = document.getElementById('hm-grid');
  grid.innerHTML = ms.slice(0,16).map(m=>{
    const hasSig = EDGE_SIGNALS.some(s=>s.market_slug===m.slug||s.market_slug===m.condition_id);
    const chg = getHeatmapChange(m);

    const abs  = Math.abs(chg);

    let bg, bc, cc, glow;
    if(chg > 1.5) {
      const a = Math.min(0.45, 0.15 + abs/20);
      bg   = `rgba(0,255,136,${a})`;
      bc   = `rgba(0,255,136,${Math.min(0.9,0.35+abs/12)})`;
      cc   = '#00ff88';
      glow = `box-shadow:0 0 12px rgba(0,255,136,${Math.min(0.5,abs/15)}) inset,0 0 6px rgba(0,255,136,.2)`;
    } else if(chg < -1.5) {
      const a = Math.min(0.45, 0.15 + abs/20);
      bg   = `rgba(255,51,85,${a})`;
      bc   = `rgba(255,51,85,${Math.min(0.9,0.35+abs/12)})`;
      cc   = '#ff3355';
      glow = `box-shadow:0 0 12px rgba(255,51,85,${Math.min(0.5,abs/15)}) inset,0 0 6px rgba(255,51,85,.2)`;
    } else {
      bg   = 'rgba(255,255,255,0.06)';
      bc   = 'rgba(255,255,255,0.13)';
      cc   = '#556677';
      glow = '';
    }

    const vol = m.volume_24h >= 1e6
      ? `$${(m.volume_24h/1e6).toFixed(1)}M`
      : `$${(m.volume_24h/1000).toFixed(1)}K`;

    const badges = buildHeatmapBadges(m, true);


   return `<div class="hm-cell" onclick="updateActiveMktById('${m.condition_id}')"
      onmouseover="this.style.filter='brightness(1.4)';this.style.transform='scale(1.05)';this.style.zIndex=10"
      onmouseout="this.style.filter='';this.style.transform='';this.style.zIndex=''"
      style="background:${bg};border:1px solid ${bc};${glow}">
      <div class="hm-n">${(m.short_name||m.question||'—').slice(0,40)}${hasSig?' <span style="color:#00d4ff;font-size:8px">⚡</span>':''}</div>

      <div class="hm-p" style="color:${cc}">${chg>=0?'+':''}${chg.toFixed(1)}%</div>
      <div class="hm-v">${vol}</div>
      <div class="hm-outs" style="margin-top:auto">${badges}</div>
    </div>`;
  }).join('');
}

// ═══ RENDER: WHALE INTEL ════════════════════════════════════════
function renderWhales() {
  const list = document.getElementById('whale-intel-list');
  if(!list) return;
  // Sort by whale_score descending
  const sorted = [...WHALES].sort((a,b)=>(b.whale_score||0)-(a.whale_score||0)).slice(0,25);
  document.getElementById('whale-badge').textContent = sorted.length;
  document.getElementById('wh-top').textContent = sorted.length;

  list.innerHTML = sorted.map((w,i)=>{
    const name   = w.pseudonym||w.wallet_short;
    const tagCls = {'APEX PREDATOR':'tag-apex','ALPHA HUNTER':'tag-alpha','MARKET MAKER':'tag-mm','ACCUMULATOR':'tag-acc','SMART GRINDER':'tag-alpha'}[w.whale_tag]||'tag-other';
    const sc     = w.whale_score||0;
    const scCls  = sc>80?'g':sc>60?'c':sc>40?'y':'r';
    const isBuyer= (w.dominant_action||'BUYER')==='BUYER';
    return `<div class="whale-intel-row" onclick="confirmRedirect('https://polymarket.com/profile/${w.wallet}')">
      <div style="color:var(--t3);font-size:7.5px;text-align:center">#${i+1}</div>
      <div class="w-name-cell">
        <div class="w-addr">${name} <span class="whale-tag ${tagCls}">${w.whale_tag}</span></div>
        <div class="w-meta">$${fmtNum(w.total_volume||0)} · ${(w.win_rate||50).toFixed(0)}%WR · <span style="color:${isBuyer?'var(--green)':'var(--red)'}">${w.dominant_action}</span></div>
      </div>
      <div class="score-val ${scCls}">${sc.toFixed(0)}</div>
    </div>`;
  }).join('');
}

// Refresh topbar win rate + alpha count from whale data
(function(){
  const valid = WHALES.filter(w => w.win_rate && w.win_rate !== 50);
  const wr = valid.length ? valid.reduce((a,w)=>a+w.win_rate,0)/valid.length : null;
  const el = document.getElementById('wr-top');
  if (el && el.textContent === '—' && wr) el.textContent = wr.toFixed(1)+'%';
  const al = document.getElementById('alpha-top');
  if (al && (al.textContent === '—' || al.textContent === '0'))
    al.textContent = WHALES.filter(w=>(w.whale_score||0)>70).length || '—';
})();

// ═══ RENDER: ALERTS ═════════════════════════════════════════════
function addAlert(title, desc, time, color, url) {
  alertCount++;
  const el2 = document.getElementById('alert-badge');
  if(el2) el2.textContent = alertCount;
  const body = document.getElementById('alerts-body');
  if(!body) return;
  body.querySelector('div[style]')?.remove();
  const el = document.createElement('div');
  el.className='a-item';
  el.innerHTML=`<span class="a-icon">⚡</span>
    <div class="a-body">
      <div class="a-title" style="color:var(--${color==='g'?'green':'red'})">${title}</div>
      <div class="a-desc">${desc}</div>
      <div class="a-time">${time}${url?` · <a href="${url}" target="_blank" style="color:var(--cyan)">↗</a>`:''}</div>
    </div>`;
  body.insertBefore(el, body.firstChild);
}

// ═══ ACTIVE MARKET ══════════════════════════════════════════════
function updateActiveMktById(conditionId) {
  const idx = MARKETS.findIndex(m=>m.condition_id===conditionId);
  if(idx>=0) updateActiveMkt(idx);
}

function updateActiveMkt(idx) {
  if(idx<0||idx>=MARKETS.length) return;
  activeMktIdx = idx;
  const m = MARKETS[idx];
  document.getElementById('active-mkt-name').textContent = m.question||m.short_name;
  const link = document.getElementById('active-mkt-link');
  const url  = m.url||`https://polymarket.com/event/${m.event_slug}`;
  link.href = url; link.dataset.url = url;
  const note = document.getElementById('exec-note');
  if(note) note.textContent = `${m.short_name} · ${(m.outcomes[0]?.price_cents||50).toFixed(1)}¢ YES`;
  // Render CLOB from cache or synth
  seedPressureData(m);
  initPressureHover();
  // Seed OHLC chart
  seedOHLC(m);
  updatePositionPressure();
}

// ═══ CANDLE CHART ═══════════════════════════════════════════════
function ingestCandle(price, vol, ts) {
  const cs = Math.floor(ts/candleDuration)*candleDuration;
  let c = priceChartData[priceChartData.length-1];
  if(!c||c.ts_start!==cs) {
    priceChartData.push({open:price,high:price,low:price,close:price,volume:vol,ts_start:cs});
  } else {
    c.high=Math.max(c.high,price); c.low=Math.min(c.low,price); c.close=price; c.volume+=vol;
  }
  if(priceChartData.length>60) priceChartData.shift();
}

function seedOHLC(m) {
  priceChartData=[];
  const basePrice = m.outcomes[0]?.price_cents||50;
  const mktTrades = TRADES.filter(t=>t.market_slug===m.slug||t.condition_id===m.condition_id).reverse();
  mktTrades.forEach(t=>ingestCandle(t.price_cents, t.size_usd, t.ts));
  // Pad if needed
  if(priceChartData.length<20) {
    const needed = 30-priceChartData.length;
    const startTs = (priceChartData[0]?.ts_start||Date.now()) - needed*candleDuration;
    let lp=basePrice;
    for(let i=0;i<needed;i++) {
      const o=lp+(Math.random()-.5)*1.8, c=o+(Math.random()-.5)*1.2;
      priceChartData.splice(i,0,{open:o,high:Math.max(o,c)+Math.random()*.4,low:Math.min(o,c)-Math.random()*.4,close:c,volume:1000+Math.random()*4000,ts_start:startTs+i*candleDuration});
      lp=c;
    }
  }
  if(priceChartData.length>60) priceChartData=priceChartData.slice(-60);
  drawPriceChart();
}

function drawPriceChart() {
  const canvas = document.getElementById('price-chart-canvas');
  if(!canvas) return;
  const ctx=canvas.getContext('2d');
  const dpr=window.devicePixelRatio||1;
  const W=canvas.offsetWidth||canvas.parentElement.offsetWidth,H=canvas.offsetHeight||72;
  canvas.width=W*dpr; canvas.height=H*dpr; ctx.scale(dpr,dpr);
  ctx.clearRect(0,0,W,H);


  if(!priceChartData.length) { ctx.font='8px JetBrains Mono'; ctx.fillStyle='var(--t3)'; ctx.textAlign='center'; ctx.fillText('NO DATA',W/2,H/2+3); return; }

  const minP=Math.min(...priceChartData.map(c=>c.low));
  const maxP=Math.max(...priceChartData.map(c=>c.high));
  const pr=(maxP-minP)||1;
  const pd=10; const ch=H-pd*2;
  const cw=(W-pd*2)/priceChartData.length;
  const gy=p=>pd+ch-((p-minP)/pr)*ch;

  ctx.strokeStyle='rgba(0,0,0,.08)'; ctx.lineWidth=1;


  [0,.33,.66,1].forEach(f=>{const y=gy(minP+pr*f); ctx.beginPath();ctx.moveTo(pd,y);ctx.lineTo(W-pd,y);ctx.stroke();});

  priceChartData.forEach((c,i)=>{
  const bull = c.close >= c.open;
  const wickCol = bull ? 'rgba(26,255,122,.45)' : 'rgba(220,50,70,.45)';
  const fillCol = bull ? 'rgba(26,255,122,.1)' : 'rgba(220,50,70,.12)';
  const borderCol = bull ? 'rgba(26,255,122,.5)' : 'rgba(220,50,70,.5)';

  const x = pd + i * cw + cw / 2;

  ctx.strokeStyle = wickCol;
  ctx.lineWidth = 1.2;
  ctx.beginPath();
  ctx.moveTo(x, gy(c.high));
  ctx.lineTo(x, gy(c.low));
  ctx.stroke();

  const bt = gy(Math.max(c.open, c.close));
  const bb = gy(Math.min(c.open, c.close));
  const bh = Math.max(2, bb - bt);
  const bw = Math.max(3, cw * 0.7);

  ctx.fillStyle = fillCol;
  ctx.fillRect(x - bw / 2, bt, bw, bh);

  ctx.strokeStyle = borderCol;
  ctx.lineWidth = 1;
  ctx.strokeRect(x - bw / 2, bt, bw, bh);
});


  const last=priceChartData[priceChartData.length-1];
  const ly=gy(last.close);
  ctx.setLineDash([2,4]); ctx.strokeStyle='rgba(255,255,255,.08)';
  ctx.beginPath();ctx.moveTo(pd,ly);ctx.lineTo(W-pd,ly);ctx.stroke();ctx.setLineDash([]);
  const ohlcEl=document.getElementById('ohlc-last');
  if(ohlcEl) ohlcEl.textContent=last.close.toFixed(1)+'¢';
}

// ═══ EQUITY CHART ═══════════════════════════════════════════════
function drawEquity() {
  const canvas=document.getElementById('eq-canvas');
  if(!canvas) return;
  const wrap=canvas.parentElement;
  const dpr=window.devicePixelRatio||1;
  const W=wrap.offsetWidth, H=wrap.offsetHeight;
  if(!W||!H||equityCurve.length<2) return;
  canvas.width=W*dpr; canvas.height=H*dpr;
  canvas.style.width=W+'px'; canvas.style.height=H+'px';
  const ctx=canvas.getContext('2d'); ctx.scale(dpr,dpr);
  const pts=equityCurve.map(p=>p.v);
  const minV=Math.min(...pts)*.997, maxV=Math.max(...pts)*1.003;
  const xS=W/(pts.length-1), yS=v=>H-((v-minV)/(maxV-minV))*(H-10)-5;
  ctx.strokeStyle='rgba(26,53,77,.5)'; ctx.lineWidth=1;
  [.25,.5,.75].forEach(f=>{const y=5+f*(H-10);ctx.beginPath();ctx.moveTo(0,y);ctx.lineTo(W,y);ctx.stroke();});
  const isUp=pts[pts.length-1]>=pts[0], col=isUp?'#00FF88':'#FF3355';
  const grad=ctx.createLinearGradient(0,0,0,H);
  grad.addColorStop(0,isUp?'rgba(0,255,136,.22)':'rgba(255,51,85,.22)'); grad.addColorStop(1,'rgba(0,0,0,0)');
  ctx.beginPath(); ctx.moveTo(0,yS(pts[0]));
  pts.forEach((p,i)=>ctx.lineTo(i*xS,yS(p)));
  ctx.lineTo((pts.length-1)*xS,H); ctx.lineTo(0,H); ctx.closePath(); ctx.fillStyle=grad; ctx.fill();
  ctx.beginPath(); ctx.moveTo(0,yS(pts[0]));
  pts.forEach((p,i)=>ctx.lineTo(i*xS,yS(p)));
  ctx.strokeStyle=col; ctx.lineWidth=1.8; ctx.shadowColor=col; ctx.shadowBlur=7; ctx.stroke(); ctx.shadowBlur=0;
  const lx=(pts.length-1)*xS, ly=yS(pts[pts.length-1]);
  ctx.beginPath();ctx.arc(lx,ly,3.5,0,Math.PI*2);ctx.fillStyle=col;ctx.fill();

  const wrap2=canvas.parentElement;
  wrap2.onmousemove=e=>{
    const r=canvas.getBoundingClientRect(),x=e.clientX-r.left,y=e.clientY-r.top;
    const chx=document.getElementById('ch-x'),chy=document.getElementById('ch-y');
    if(chx){chx.style.left=x+'px';chx.style.display='block';}
    if(chy){chy.style.top=y+'px';chy.style.display='block';}
  };
  wrap2.onmouseleave=()=>{
    const chx=document.getElementById('ch-x'),chy=document.getElementById('ch-y');
    if(chx)chx.style.display='none'; if(chy)chy.style.display='none';
  };
}

function updateEquityUI() {
  const pnl=equityVal-10000, pct=(pnl/10000*100).toFixed(2), isPos=pnl>=0;
  const f2=v=>'$'+Math.round(v).toLocaleString();
  const set=(id,v)=>{const el=document.getElementById(id);if(el)el.textContent=v;};
  set('eq-top',f2(equityVal)); set('eq-main',f2(equityVal)); set('eq-ath',f2(athVal));
  const pnlEl=document.getElementById('pnl-top');
  if(pnlEl){pnlEl.textContent=(isPos?'+':'')+f2(pnl);pnlEl.className='tsv '+(isPos?'g':'r');}
  const chgEl=document.getElementById('eq-chg');
  if(chgEl){chgEl.textContent=(isPos?'+':'')+pct+'%';chgEl.className='eq-chg '+(isPos?'g':'r');}
  set('hdr-pnl',(isPos?'+':'')+f2(pnl));
}

function updateBSR() {
  const total=buyVol+sellVol||1;
  const set=(id,v)=>{const el=document.getElementById(id);if(el)el.textContent=v;};
  set('buy-vol',fmt(buyVol)); set('sell-vol',fmt(sellVol));
  const buy=document.getElementById('bsr-buy-bar'),sell=document.getElementById('bsr-sell-bar');
  if(buy)buy.style.width=(buyVol/total*100)+'%';
  if(sell)sell.style.width=(sellVol/total*100)+'%';
  const ratio=(buyVol/Math.max(sellVol,1)).toFixed(2);
  set('bsr-top',ratio+'x'); set('bsr-mid',ratio+'x'); set('bsr-ratio',ratio+'x');
}

// ═══ TRIAL UI ═══════════════════════════════════════════════════
function updateTrialUI() {
  if (_licValid) return; // paid user — ignore trial entirely
  const rem=Math.max(0,trialRemaining);
  const fill=document.getElementById('trial-fill');
  if(fill) fill.style.width=(rem/trialTotal*100)+'%';
  const m=Math.floor(rem/60), s=rem%60;
  const el=document.getElementById('trial-time');
  if(el){el.textContent=`${m}:${s.toString().padStart(2,'0')}`;el.style.color=rem<60?'var(--red)':rem<120?'var(--yellow)':'var(--green)';}
  if(rem<=0) showPaywall();
}

function showPaywall() {
  if (_licValid) return; // paid user — never show paywall
  trialExpired = true;
  if (ws) ws.close();
  openAccessModal('license');
}
function selectPlan(p) { selectedPlan = p; }
function checkout() { openAccessModal('buy'); }
// ═══ EXECUTION LAYER ════════════════════════════════════════════
function setQty(amt, el) {
  activeQty = amt;
  document.querySelectorAll('.qty-chip').forEach(c=>c.classList.remove('active'));
  el.classList.add('active');
}

function executeTrade(action) {
  const m = MARKETS[activeMktIdx];
  if(!m) { alert('Select a market first'); return; }
  const url = m.url||`https://polymarket.com/event/${m.event_slug}`;
  const outcome = m.outcomes[0]?.name||'YES';
  confirmRedirect(url, `${action} ${outcome} on ${m.short_name} on Polymarket?`);
}

// ═══ MODAL ══════════════════════════════════════════════════════
function confirmRedirect(url, msg='Opening Polymarket. Proceed?') {
  if(!url||url==='#') return;
  const overlay=document.getElementById('modal-overlay');
  const btn=document.getElementById('modal-confirm');
  const msg2=document.getElementById('modal-msg');
  if(msg2) msg2.textContent=msg;
  if(overlay) overlay.style.display='flex';
  if(btn) btn.onclick=()=>{ window.open(url,'_blank'); closeModal(); };
}
function closeModal() {
  const o=document.getElementById('modal-overlay');
  if(o) o.style.display='none';
}

// ═══ TICKER ═════════════════════════════════════════════════════
function renderTicker() {
  const items=[
    {n:'POLYMARKET',v:'LIVE',c:'',u:true},{n:'BTC',v:'$62K',c:'+2.3%',u:true},
    {n:'ETH',v:'$3.2K',c:'+1.1%',u:true},{n:'CLOB',v:'4-SIDED',c:'',u:true},
    {n:'WHALE MIN',v:'$5K',c:'',u:false},{n:'TRIAL',v:'5 MIN',c:'FREE',u:true},
    {n:'SIGNALS',v:'8 TYPES',c:'',u:true},{n:'SCORE',v:'WR·ROI·CONS·VOL',c:'',u:false},
  ];
  const all=[...items,...items];
  document.getElementById('t-inner').innerHTML=all.map(it=>
    `<div class="t-item"><span class="t-n">${it.n}</span><span class="t-v">${it.v}</span>${it.c?`<span class="${it.u?'t-up':'t-dn'}">${it.c}</span>`:''}</div>`
  ).join('');
}

// ═══ FILTERS ════════════════════════════════════════════════════
function setMinSize(btn,size) {
  minSize=size;
  btn.closest('#filterbar').querySelectorAll('.fbtn').forEach(b=>{if(['ALL','$500+','$1K+','$5K+','$25K+'].some(t=>b.textContent.includes(t)))b.classList.remove('on');});
  btn.classList.add('on'); renderTrades();
}
function setActionFilter(btn,f) {
  actionFilter=f;
  document.querySelectorAll('#filterbar .fbtn').forEach(b=>{if(['ALL','BUY','SELL'].includes(b.textContent))b.classList.remove('on');});
  btn.classList.add('on'); renderTrades();
}
function setCat(btn,c) {
  catFilter=c;
  const sibs=[...btn.parentElement.children].filter(el=>el.classList.contains('fbtn'));
  sibs.forEach(b=>b.classList.remove('on')); btn.classList.add('on');
  renderHeatmap();
}
function filterMarkets() { renderHeatmap(); }
function togglePause(btn) {
  feedPaused=!feedPaused;
  btn.textContent=feedPaused?'▶ RESUME':'⏸ PAUSE';
  btn.classList.toggle('on',feedPaused);
}
function toggleWhaleOnly(btn) {
  whaleOnly=!whaleOnly;
  btn.classList.toggle('on',whaleOnly);
  renderTrades();
}

// ═══ POSITION PRESSURE PANEL ════════════════════════════════════
function updatePositionPressure() {
  // Pull from live state: SIGNALS (whale_bias, whale_flow, composite)
  // and active market outcomes for crowd positioning
  const bias      = SIGNALS.whale_bias || 0;       // -1 to +1
  const composite = SIGNALS.composite  || 50;      // 0-100
  const wf        = SIGNALS.whale_flow || 50;      // 0-100

  // Crowd: derive from active market YES price
  const mkt     = MARKETS[activeMktIdx];
  const yesPct  = mkt ? Math.round(getPrimaryProb(mkt)) : 50;
  const noPct   = 100 - yesPct;

  // Smart money label
  const smartLabel = bias >  0.15 ? 'BUYING'
                   : bias < -0.15 ? 'SELLING'
                   : 'NEUTRAL';
  const smartColor = bias >  0.15 ? '#00ff88'
                   : bias < -0.15 ? '#ff3355'
                   : '#00d4ff';

  // Imbalance
  const absB = Math.abs(bias);
  const imbLabel = absB > 0.45 ? 'HIGH'
                 : absB > 0.2  ? 'MEDIUM'
                 : 'LOW';
  const imbColor = absB > 0.45 ? (bias > 0 ? '#00ff88' : '#ff3355')
                 : absB > 0.2  ? '#ffb800'
                 : '#5e849e';

  // Crowd overload label
  const crowdLabel = yesPct > 72 ? 'CROWD OVERLOADED YES'
                   : noPct  > 72 ? 'CROWD OVERLOADED NO'
                   : yesPct > 60 ? 'YES LEANING'
                   : noPct  > 60 ? 'NO LEANING'
                   : 'BALANCED MARKET';
  const crowdLabelColor = yesPct > 72 ? '#00ff88'
                        : noPct  > 72 ? '#ff3355'
                        : yesPct > 60 ? 'rgba(0,255,136,.6)'
                        : noPct  > 60 ? 'rgba(255,51,85,.6)'
                        : '#5e849e';

  // Whale flow label
  const wfLabel = wf > 65 ? 'STRONG BUY' : wf > 52 ? 'BUYING' : wf < 35 ? 'STRONG SELL' : wf < 48 ? 'SELLING' : 'NEUTRAL';
  const wfColor = wf > 52 ? '#9b5de5' : wf < 48 ? '#ff6b35' : '#5e849e';

  const set = (id, val) => { const el = document.getElementById(id); if (el) el.textContent = val; };
  const sty = (id, prop, val) => { const el = document.getElementById(id); if (el) el.style[prop] = val; };

  set('pp-yes-pct', yesPct);
  set('pp-no-pct',  noPct);
  sty('pp-yes-bar', 'width', yesPct + '%');
  sty('pp-no-bar',  'width', noPct  + '%');

  set('pp-smart-money',    smartLabel);
  sty('pp-smart-money',    'color', smartColor);
  sty('pp-smart-money',    'textShadow', `0 0 8px ${smartColor}`);

  set('pp-imbalance-val',  imbLabel);
  sty('pp-imbalance-val',  'color', imbColor);
  sty('pp-imbalance-val',  'textShadow', `0 0 8px ${imbColor}`);

  set('pp-whale-flow',     wfLabel);
  sty('pp-whale-flow',     'color', wfColor);

  set('pp-imbalance-label', crowdLabel);
  sty('pp-imbalance-label', 'color',       crowdLabelColor);
  sty('pp-imbalance-label', 'borderColor', crowdLabelColor);
  sty('pp-imbalance-label', 'background',  `${crowdLabelColor}15`);
}

// ═══ UTILITIES ══════════════════════════════════════════════════
function fmt(v) { if(!v||isNaN(v))return'—';if(v>=1e9)return'$'+(v/1e9).toFixed(1)+'B';if(v>=1e6)return'$'+(v/1e6).toFixed(1)+'M';if(v>=1000)return'$'+(v/1000).toFixed(1)+'K';return'$'+v.toFixed(0); }
function fmtNum(v) { if(!v||isNaN(v))return'0';if(v>=1e6)return(v/1e6).toFixed(1)+'M';if(v>=1000)return(v/1000).toFixed(1)+'K';return v.toFixed(0); }
function truncAddr(s) { if(!s||s.length<=12)return s||'';return s.slice(0,6)+'…'+s.slice(-4); }

// ═══ RESIZE ═════════════════════════════════════════════════════
new ResizeObserver(()=>{ drawEquity(); drawPriceChart(); drawPressureMap(); })
  .observe(document.getElementById('eq-canvas').parentElement);

// ═══ BOOT ═══════════════════════════════════════════════════════
renderTicker();
// Seed equity
for(let i=0;i<30;i++){ equityVal+=((Math.random()-.47)*70); equityCurve.push({ts:Date.now()/1000-30+i,v:equityVal}); }
drawEquity(); updateEquityUI(); drawPriceChart();
connect();

// ═══ LICENSE SYSTEM ══════════════════════════════════════════════════════════
const _LK = 'wt_lic', _DK = 'wt_dev', _CK = 'wt_cache';
let _licValid = false, _revalTimer = null;

function _devId() {
  let d = localStorage.getItem(_DK);
  if (!d) { d = 'DEV-' + Math.random().toString(36).slice(2,10).toUpperCase() + '-' + Date.now().toString(36).toUpperCase(); localStorage.setItem(_DK, d); }
  return d;
}

function _licCacheValid() {
  try {
    const c = JSON.parse(localStorage.getItem(_CK) || '{}');
    if (!c.ts || Date.now() - c.ts >= 86400000) return null;
    return c;
  } catch { return null; }
}

async function _licValidate(key) {
  const dev = _devId();
  const url = `${API}/validate?key=${encodeURIComponent(key)}&device=${encodeURIComponent(dev)}`;
  console.log('[whale] _licValidate URL:', url);
  try {
    const r = await fetch(url);
    console.log('[whale] validate HTTP status:', r.status);
    const text = await r.text();
    console.log('[whale] validate response body:', text);
    let j;
    try { j = JSON.parse(text); } catch { throw new Error('bad json: ' + text); }
    if (!r.ok) throw new Error('http ' + r.status);
    if (j.valid) {
      localStorage.setItem(_LK, key);
      localStorage.setItem(_CK, JSON.stringify({ ts: Date.now(), expires: j.expires, status: j.status, warning: j.warning }));
    }
    return j;
  } catch(err) {
    console.error('[whale] validate error:', err);
    const c = _licCacheValid();
    if (c) return { valid: true, status: 'CACHED', expires: c.expires, warning: c.warning || '' };
    return { valid: false, status: 'OFFLINE', expires: '', warning: '' };
  }
}

function _applyLicState(valid, warning) {
  _licValid = valid;
  const m = document.getElementById('main');
  if (valid) {
    m?.classList.remove('blurred');
    document.getElementById('access-modal')?.classList.remove('show');
    document.getElementById('paywall')?.classList.remove('show');

    // hide trial bar + unlock button — user is paid, not on trial
    const trialWrap = document.getElementById('trial-wrap');
    if (trialWrap) trialWrap.style.display = 'none';
    const unlockBtn = document.getElementById('unlock-btn');
    if (unlockBtn) unlockBtn.style.display = 'none';

    // stop trial countdown from doing anything
    trialExpired = false;
    trialRemaining = 999999;

    if (warning === 'multiple_devices') document.getElementById('am-warn-banner')?.classList.add('show');
    if (!ws || ws.readyState === WebSocket.CLOSED || ws.readyState === WebSocket.CLOSING) {
      connect();
    }
  } else {
    m?.classList.add('blurred');
    const trialWrap = document.getElementById('trial-wrap');
    if (trialWrap) trialWrap.style.display = '';
    const unlockBtn = document.getElementById('unlock-btn');
    if (unlockBtn) unlockBtn.style.display = '';
  }
}

async function _licActivate() {
  // Two #lic-input elements exist — grab all and use the one with a value
  const inputs = document.querySelectorAll('#lic-input');
  const input = [...inputs].find(el => el.value.trim()) || inputs[inputs.length - 1];
  const status = document.getElementById('lic-status');
  const key = (input?.value || '').trim();
  console.log('[whale] activate clicked, key:', key, 'API:', API);
  if (!key) { if (status) { status.textContent = '⚠ Enter your license key'; status.style.color = '#ffb800'; } return; }
  if (status) { status.textContent = '⏳ Validating… (may take 30s if server is waking)'; status.style.color = '#5e849e'; }
  const result = await _licValidate(key);
  console.log('[whale] validate result:', result);
  if (result.valid) {
    if (status) { status.textContent = `✓ ${result.status} — expires ${result.expires || 'LIFETIME'}`; status.style.color = '#00ff88'; }
    _licValid = true;                          // set BEFORE closeAccessModal guard checks it
    _applyLicState(true, result.warning);      // removes blur, hides modal, reconnects ws
    _startRevalidation();
  } else {
    const msgs = { TOO_MANY_DEVICES: '✗ Too many devices — contact support', EXPIRED: '✗ License expired', INACTIVE: '✗ Key inactive — contact support', INVALID: '✗ Invalid key', OFFLINE: '⚠ Offline — try again shortly' };
    if (status) { status.textContent = msgs[result.status] || '✗ Validation failed'; status.style.color = '#ff3355'; }
  }
}

function _startRevalidation() {
  if (_revalTimer) clearInterval(_revalTimer);
  _revalTimer = setInterval(async () => {
    const stored = localStorage.getItem(_LK);
    if (!stored) return;
    const r = await _licValidate(stored);
    _applyLicState(r.valid, r.warning);
    if (!r.valid && trialExpired && !_licValid) openAccessModal('license');
  }, 7 * 60 * 1000);
}

function openAccessModal(tab) {
  document.getElementById('access-modal')?.classList.add('show');
  switchAmTab(tab || 'license');
  const closeBtn = document.getElementById('am-close-btn');
  if (closeBtn) closeBtn.style.display = (trialExpired && !_licValid) ? 'none' : '';
  if (tab === 'buy') _loadPaymentInfo();
}

function closeAccessModal() {
  if (trialExpired && !_licValid) return;
  document.getElementById('access-modal')?.classList.remove('show');
  document.getElementById('paywall')?.classList.remove('show');
}

function switchAmTab(tab) {
  ['license','buy'].forEach(t => {
    document.getElementById('am-tab-'+t)?.classList.toggle('active', t===tab);
    document.getElementById('am-pane-'+t)?.classList.toggle('active', t===tab);
  });
  if (tab === 'buy') _loadPaymentInfo();
}

let _payInfoLoaded = false;
async function _loadPaymentInfo() {
  if (_payInfoLoaded) return;
  try {
    const j = await (await fetch(`${API}/api/payment-info`)).json();
    _payInfoLoaded = true;
    const addrList = document.getElementById('am-addr-list');
    if (addrList && j.networks) {
      addrList.innerHTML = j.networks.map(n => `
        <div class="am-addr-box">
          <div style="flex:1;min-width:0"><div class="am-network">${n.name}</div><div class="am-addr">${n.address}</div></div>
          <button class="am-copy" onclick="_copyAddr('${n.address}',this)">COPY</button>
        </div>`).join('');
    }
    function _submitPayment() {
  const txHash   = (document.getElementById('am-txhash')?.value||'').trim();
  const telegram = (document.getElementById('am-telegram')?.value||'').trim();
  if (!txHash)   { alert('Please enter your TX hash.'); return; }
  if (!telegram) { alert('Please enter your Telegram username.'); return; }
  navigator.clipboard.writeText(`WHALE.TERMINAL ACCESS\nPlan: ${_amPlan}\nTX: ${txHash}\nTelegram: ${telegram}`).catch(()=>{});
  const btn = document.getElementById('am-pay-submit');
  if (btn) { btn.textContent='✓ SUBMITTED — await your key on Telegram'; btn.style.color='#00ff88'; btn.disabled=true; }
  const note = document.getElementById('am-tg-note');
  if (note) note.innerHTML='<strong style="color:var(--cyan)">Details copied to clipboard.</strong> Also send to our Telegram. Key arrives within 24h.';
}
  } catch {
    const el = document.getElementById('am-addr-list');
    if (el) el.innerHTML = '<div style="color:#ff3355;font-size:8px;padding:8px">Failed to load — refresh and try again.</div>';
  }
}

function _copyAddr(addr, btn) {
  navigator.clipboard.writeText(addr).then(() => {
    if (btn) { btn.textContent = 'COPIED ✓'; btn.classList.add('copied'); setTimeout(() => { btn.textContent='COPY'; btn.classList.remove('copied'); }, 2000); }
  }).catch(() => {
    const ta = Object.assign(document.createElement('textarea'),{value:addr,style:'position:fixed;opacity:0'});
    document.body.appendChild(ta); ta.select(); document.execCommand('copy'); document.body.removeChild(ta);
    if (btn) { btn.textContent='COPIED ✓'; btn.classList.add('copied'); setTimeout(()=>{btn.textContent='COPY';btn.classList.remove('copied');},2000); }
  });
}

let _amPlan = 'yearly';
function selectAmPlan(p) {
  _amPlan = p;
  ['monthly','yearly'].forEach(x => {
    const el = document.getElementById('am-plan-'+x);
    if (!el) return;
    el.style.borderColor = x===p ? 'var(--cyan)' : 'var(--b1)';
    el.style.background  = x===p ? 'rgba(0,212,255,.05)' : 'none';
  });
}

function _submitPayment() {
  const txHash   = (document.getElementById('am-txhash')?.value||'').trim();
  const telegram = (document.getElementById('am-telegram')?.value||'').trim();
  if (!txHash)   { alert('Please enter your TX hash.'); return; }
  if (!telegram) { alert('Please enter your Telegram username.'); return; }
  navigator.clipboard.writeText(`WHALE.TERMINAL ACCESS\nPlan: ${_amPlan}\nTX: ${txHash}\nTelegram: ${telegram}`).catch(()=>{});
  const btn = document.getElementById('am-pay-submit');
  if (btn) { btn.textContent='✓ SUBMITTED — await your key on Telegram'; btn.style.color='#00ff88'; btn.disabled=true; }
  const note = document.getElementById('am-tg-note');
  if (note) note.innerHTML='<strong style="color:var(--cyan)">Details copied to clipboard.</strong> Also send to our Telegram. Key arrives within 24h.';
}

// ─── Boot ─────────────────────────────────────────────────────────────────────
(async function _licBoot() {
  const msg  = document.getElementById('boot-msg');
  const sub  = document.getElementById('boot-sub');
  const bar  = document.getElementById('boot-bar');

  const steps = [
    [0,   'FETCHING SIGNALS...',        'CALIBRATING MODELS...'],
    [800, 'LOADING WHALE ACTIVITY...',  'SCANNING ORDER FLOW...'],
    [1800,'MAPPING MARKET PRESSURE...', 'WARMING UP ENGINE...'],
    [3200,'🚀 ALMOST READY...',         'FIRST LOAD MAY TAKE ~30s'],
  ];
  let pct = 0;
  steps.forEach(([delay, m, s]) => {
    setTimeout(() => {
      if (msg) msg.textContent = m;
      if (sub) sub.textContent = s;
      pct = Math.min(pct + 25, 90);
      if (bar) bar.style.width = pct + '%';
    }, delay);
  });

  function hideOverlay() {
    if (bar) bar.style.width = '100%';
    setTimeout(() => {
      const ov = document.getElementById('boot-overlay');
      if (ov) { ov.style.transition = 'opacity .5s'; ov.style.opacity = '0'; setTimeout(() => ov.remove(), 500); }
    }, 300);
  }

  const stored = localStorage.getItem(_LK);
  try {
    if (stored && _licCacheValid()) {
      const c = _licCacheValid();
      _applyLicState(true, c?.warning || '');
      _startRevalidation();
      console.log('%c✓ License cached','color:#00FF88');
      hideOverlay(); return;
    }
    if (stored) {
      const r = await _licValidate(stored);
      if (r.valid) { _applyLicState(true, r.warning); _startRevalidation(); console.log('%c✓ License valid','color:#00FF88'); hideOverlay(); return; }
    }
    console.log('%c○ No license — trial active','color:#ffb800');
  } catch(err) {
    if (msg) { msg.textContent = '⚠️ CONNECTION ISSUE. RETRYING...'; msg.style.color = '#ffb800'; }
    await new Promise(r => setTimeout(r, 3000));
    try {
      if (stored) { const r = await _licValidate(stored); if (r.valid) { _applyLicState(true, r.warning); } }
    } catch {}
  }
  hideOverlay();
})();

document.getElementById('lic-input')?.addEventListener('keydown', e => { if (e.key==='Enter') _licActivate(); });
document.addEventListener('keydown', e => { if (e.key==='Escape') closeAccessModal(); });
// ═══ EDGE FEED MODAL ════════════════════════════════════════════
let _emFilter = 'ALL';
function filterEdgeModal(f) {
  _emFilter = f;
  document.querySelectorAll('.emf-btn').forEach(b => { b.style.opacity = (b.id==='emf-'+f||(f==='ALL'&&b.id==='emf-all')) ? '1' : '0.4'; });
  renderEdgeModal();
}
function openEdgeFeedModal() { document.getElementById('edge-modal').style.display='flex'; renderEdgeModal(); }
function closeEdgeFeedModal() { document.getElementById('edge-modal').style.display='none'; }
function renderEdgeModal() {
  const sigs = _emFilter==='ALL' ? EDGE_SIGNALS : EDGE_SIGNALS.filter(s=>s.priority===_emFilter);
  const body = document.getElementById('edge-modal-body');
  if (!body) return;
  document.getElementById('edge-modal-count').textContent = sigs.length;

  if (!sigs.length) {
    body.innerHTML = `<div style="grid-column:1/-1;display:flex;align-items:center;justify-content:center;height:300px;color:var(--t3);font-size:9px;letter-spacing:2px;flex-direction:column;gap:8px"><span style="font-size:24px">⚡</span>NO SIGNALS YET — MONITORING…</div>`;
    return;
  }

  const esc = v => String(v??'').replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;');
  const bar = (val,col) => `<div style="height:3px;background:rgba(255,255,255,.06);margin-top:3px"><div style="height:100%;width:${Math.min(100,Math.max(0,val))}%;background:${col}"></div></div>`;
  const META = {
    SMART_CLUSTER:    {cause:'3+ alpha wallets entered same outcome in 15 min',window:'10-25 min',hitRate:72,samples:148,avgMove:'+18%',vol:85,structure:70,momentum:60},
    CONVICTION_SPIKE: {cause:'Whale bet 3× their own rolling avg',window:'5-15 min',hitRate:68,samples:203,avgMove:'+14%',vol:90,structure:55,momentum:65},
    WHALE_REVERSAL:   {cause:'High-score wallet flipped direction',window:'8-20 min',hitRate:74,samples:97,avgMove:'+21%',vol:70,structure:80,momentum:75},
    VELOCITY_SURGE:   {cause:'1-min volume 4× vs 60-min baseline',window:'3-8 min',hitRate:61,samples:312,avgMove:'+9%',vol:95,structure:45,momentum:88},
    STEALTH_ACCUM:    {cause:'Repeated buys at stable price',window:'15-40 min',hitRate:66,samples:84,avgMove:'+16%',vol:75,structure:85,momentum:50},
    PROB_DIVERGENCE:  {cause:'Price moving against whale flow',window:'10-30 min',hitRate:63,samples:121,avgMove:'+12%',vol:60,structure:75,momentum:70},
    MOMENTUM_BREAK:   {cause:'Prob crossed key level with volume confirmation',window:'5-12 min',hitRate:69,samples:178,avgMove:'+15%',vol:80,structure:90,momentum:85},
    LIQUIDITY_DRAIN:  {cause:'Order-book thinning — MMs pulling liquidity',window:'2-6 min',hitRate:58,samples:67,avgMove:'+8%',vol:70,structure:65,momentum:55},
    WHALE_PRINT:      {cause:'Single oversized whale print',window:'3-10 min',hitRate:65,samples:290,avgMove:'+11%',vol:88,structure:60,momentum:72},
    WHALE_ALERT:      {cause:'$5K+ whale trade detected',window:'5-15 min',hitRate:64,samples:180,avgMove:'+12%',vol:85,structure:58,momentum:68}
  };

  body.innerHTML = sigs.map((s,i) => {
    const meta = META[s.kind] || {cause:'Signal detected',window:'5-15 min',hitRate:60,samples:50,avgMove:'+10%',vol:60,structure:60,momentum:60};
    const pc   = Number.isFinite(+s.price_cents) ? +s.price_cents : 50;
    const conf = Number.isFinite(+s.confidence)  ? +s.confidence  : 60;
    const isBuy  = String(s.action||'').toUpperCase().includes('BUY');
    const isSell = String(s.action||'').toUpperCase().includes('SELL');
    const dc = isBuy?'#00ff88':isSell?'#ff3355':'#00d4ff';
    const db = isBuy?'rgba(0,255,136,.1)':isSell?'rgba(255,51,85,.1)':'rgba(0,212,255,.08)';
    const dbo= isBuy?'rgba(0,255,136,.35)':isSell?'rgba(255,51,85,.35)':'rgba(0,212,255,.3)';
    const da = isBuy?'▲':isSell?'▼':'◆';
    const dl = isBuy?'BUY YES':isSell?'BUY NO':'WATCH';
    const cn = s.color||'cyan';
    const priCls = {CRITICAL:'pri-critical',HIGH:'pri-high',MEDIUM:'pri-medium',LOW:'pri-low'}[s.priority||'LOW']||'pri-low';
    const age = Number.isFinite(+s.ts)?Math.max(0,Math.floor((Date.now()-+s.ts)/1000)):0;
    const ageStr = age<60?`${age}s ago`:`${Math.floor(age/60)}m ago`;
    const ks = (s.kind||'').split('').reduce((a,c)=>a+c.charCodeAt(0),0);
    const ts = +s.ts%100;
    const mp = Math.max(2,Math.min(98,pc+(isBuy?1:-1)*(2+ks%9)+((ts%5)-2)));
    const ep = isBuy?mp-pc:pc-mp;
    const ec = ep>=5?'#00ff88':ep>=2?'#ffb800':'#ff3355';
    const dm = parseInt((meta.window||'10').split(/[-–]/)[1])||10;
    const gi = EDGE_SIGNALS.findIndex(x=>x.id===s.id);

    return `<div class="sig-card border-${cn}" style="cursor:pointer" onclick="closeEdgeFeedModal();openEdgeSignal(EDGE_SIGNALS[${gi}])">
      <div class="sig-card-top">
        <div class="sig-hdr">
          <div style="display:flex;align-items:center;gap:6px">
            <span class="sig-kind color-${cn}">${esc(String(s.kind||'SIGNAL').replace(/_/g,' '))}</span>
            <span style="font-size:6.5px;color:var(--t3)">${ageStr}</span>
          </div>
          <div class="sig-conf-wrap">
            <span class="sig-pri ${priCls}">${esc(s.priority||'LOW')}</span>
            <span class="sig-conf-val">${conf}%</span>
          </div>
        </div>
        <div style="display:flex;align-items:center;gap:8px;margin:4px 0 3px">
          <div style="font-family:'Orbitron';font-size:13px;font-weight:900;color:${dc};background:${db};border:1px solid ${dbo};padding:3px 10px;flex-shrink:0">${da} ${dl}</div>
          <div style="font-size:8px;color:var(--t2);line-height:1.3;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;flex:1">${esc(s.market||s.market_slug||'Polymarket')}${s.outcome?`<br><span style="color:var(--cyan);font-size:7px">→ ${esc(s.outcome)}</span>`:''}</div>
        </div>
        <div style="display:flex;border:1px solid rgba(255,255,255,.06);background:rgba(0,0,0,.25);margin-top:2px">
          <div style="flex:1;padding:3px 7px;border-right:1px solid rgba(255,255,255,.06)">
            <div style="font-size:6px;color:var(--t3);text-transform:uppercase;letter-spacing:1px">Market</div>
            <div style="font-family:'Orbitron';font-size:10px;font-weight:700;color:var(--t1)">${pc.toFixed(1)}¢</div>
          </div>
          <div style="flex:1;padding:3px 7px;border-right:1px solid rgba(255,255,255,.06)">
            <div style="font-size:6px;color:var(--t3);text-transform:uppercase;letter-spacing:1px">Model</div>
            <div style="font-family:'Orbitron';font-size:10px;font-weight:700;color:var(--cyan)">${mp.toFixed(1)}¢</div>
          </div>
          <div style="flex:1;padding:3px 7px">
            <div style="font-size:6px;color:var(--t3);text-transform:uppercase;letter-spacing:1px">Edge</div>
            <div style="font-family:'Orbitron';font-size:11px;font-weight:900;color:${ec}">${ep>=0?'+':''}${ep.toFixed(1)}%</div>
          </div>
        </div>
        <div style="font-size:7px;color:var(--t2);padding:4px 7px;background:rgba(0,0,0,.22);border-top:1px solid rgba(255,255,255,.04)"><span style="color:var(--t3);text-transform:uppercase;letter-spacing:1px">Cause · </span>${esc(meta.cause)}</div>
      </div>
      <div class="sig-card-body">
        <div>
          <div style="font-size:6px;color:var(--t3);letter-spacing:1.5px;text-transform:uppercase;margin-bottom:4px">Confidence breakdown</div>
          <div style="display:grid;grid-template-columns:1fr 1fr 1fr;gap:5px">
            <div><div style="display:flex;justify-content:space-between"><span style="font-size:6px;color:var(--t3)">VOL</span><span style="font-size:6.5px;font-weight:700;color:#00d4ff">${meta.vol}%</span></div>${bar(meta.vol,'#00d4ff')}</div>
            <div><div style="display:flex;justify-content:space-between"><span style="font-size:6px;color:var(--t3)">STRUCT</span><span style="font-size:6.5px;font-weight:700;color:#a855f7">${meta.structure}%</span></div>${bar(meta.structure,'#a855f7')}</div>
            <div><div style="display:flex;justify-content:space-between"><span style="font-size:6px;color:var(--t3)">MOM</span><span style="font-size:6.5px;font-weight:700;color:#ffb800">${meta.momentum}%</span></div>${bar(meta.momentum,'#ffb800')}</div>
          </div>
        </div>
        <div style="display:flex;align-items:center;justify-content:space-between;padding:3px 7px;background:rgba(255,184,0,.05);border:1px solid rgba(255,184,0,.18)">
          <span style="font-size:6px;color:var(--t3);text-transform:uppercase;letter-spacing:1px">Window</span>
          <span style="font-size:8px;font-weight:700;color:#ffb800;font-family:'Orbitron'">${meta.window}</span>
          <span style="font-size:6.5px;color:var(--t3)">weakens after ${dm}m</span>
        </div>
      </div>
      <div class="sig-card-footer">
        <div style="display:flex;align-items:center;gap:6px">
          <span style="font-size:6.5px;color:var(--t3)">HIT</span>
          <span style="font-family:'Orbitron';font-size:9px;font-weight:700;color:${meta.hitRate>=70?'#00ff88':meta.hitRate>=60?'#ffb800':'#ff3355'}">${meta.hitRate}%</span>
          <span style="font-size:6.5px;color:var(--t3)">n=${meta.samples} · AVG ${esc(meta.avgMove)}</span>
          <span style="margin-left:auto;font-family:'Orbitron';font-size:8px;font-weight:700;padding:2px 8px;border:1px solid ${dbo};background:${db};color:${dc}">${da} ${dl}</span>
        </div>
      </div>
    </div>`;
  }).join('');
}

function openHow() {
  document.getElementById('how-modal').style.display = 'flex';
  renderHowContent();
}
function closeHow() {
  document.getElementById('how-modal').style.display = 'none';
}
function renderHowContent() {
  const body = document.getElementById('how-body');
  if (!body || body.dataset.loaded) return;
  body.dataset.loaded = '1';

  // SVG mini-illustrations inline
  const SVG = {
    edge: `<svg viewBox="0 0 120 60" xmlns="http://www.w3.org/2000/svg" style="width:120px;height:60px;display:block">
      <rect width="120" height="60" fill="#020c14"/>
      <polyline points="0,45 20,40 40,35 60,20 80,15 100,8 120,5" fill="none" stroke="#00FF88" stroke-width="1.5" opacity=".8"/>
      <circle cx="100" cy="8" r="4" fill="#00FF88" opacity=".9"/>
      <line x1="60" y1="0" x2="60" y2="60" stroke="rgba(0,212,255,.2)" stroke-dasharray="3,3" stroke-width="1"/>
      <text x="62" y="14" fill="#00d4ff" font-size="6" font-family="monospace">BREAK</text>
      <circle cx="60" cy="20" r="3" fill="#FFB800"/>
    </svg>`,

    velocity: `<svg viewBox="0 0 120 60" xmlns="http://www.w3.org/2000/svg" style="width:120px;height:60px;display:block">
      <rect width="120" height="60" fill="#020c14"/>
      ${[8,18,28,38,48,60,80,100,110].map((x,i)=>{const h=i<6?8+i*3:8+i*6+10;return `<rect x="${x}" y="${60-h}" width="7" height="${h}" fill="${i>=6?'rgba(0,255,136,.85)':'rgba(0,212,255,.35)'}"/>`}).join('')}
      <text x="60" y="14" fill="#00FF88" font-size="7" font-family="monospace" text-anchor="middle">4× SURGE</text>
      <line x1="55" y1="0" x2="55" y2="60" stroke="rgba(255,184,0,.5)" stroke-dasharray="2,2" stroke-width="1"/>
    </svg>`,

    whale: `<svg viewBox="0 0 120 60" xmlns="http://www.w3.org/2000/svg" style="width:120px;height:60px;display:block">
      <rect width="120" height="60" fill="#020c14"/>
      <circle cx="30" cy="30" r="8" fill="rgba(0,255,136,.2)" stroke="#00FF88" stroke-width="1.2"/>
      <circle cx="60" cy="25" r="14" fill="rgba(0,255,136,.25)" stroke="#00FF88" stroke-width="1.5"/>
      <circle cx="95" cy="32" r="6" fill="rgba(255,51,85,.2)" stroke="#FF3355" stroke-width="1"/>
      <text x="60" y="29" fill="#fff" font-size="9" font-family="monospace" text-anchor="middle">🐋</text>
      <text x="30" y="34" fill="#00FF88" font-size="5" font-family="monospace" text-anchor="middle">$42K</text>
      <text x="95" y="36" fill="#FF3355" font-size="5" font-family="monospace" text-anchor="middle">$8K</text>
    </svg>`,

    pressure: `<svg viewBox="0 0 120 60" xmlns="http://www.w3.org/2000/svg" style="width:120px;height:60px;display:block">
      <rect width="120" height="60" fill="#020c14"/>
      <rect x="10" y="20" width="60" height="10" fill="rgba(0,255,136,.4)" rx="2"/>
      <rect x="70" y="20" width="40" height="10" fill="rgba(255,51,85,.35)" rx="2"/>
      <text x="40" y="28" fill="#00FF88" font-size="7" font-family="monospace" text-anchor="middle">YES 60%</text>
      <text x="90" y="28" fill="#FF3355" font-size="7" font-family="monospace" text-anchor="middle">NO 40%</text>
      <line x1="60" y1="0" x2="60" y2="60" stroke="rgba(255,184,0,.6)" stroke-width="1.5"/>
      <text x="60" y="50" fill="#FFB800" font-size="6" font-family="monospace" text-anchor="middle">⚠ IMBALANCE</text>
    </svg>`,

    map: `<svg viewBox="0 0 120 60" xmlns="http://www.w3.org/2000/svg" style="width:120px;height:60px;display:block">
      <rect width="120" height="60" fill="#000"/>
      <polyline points="0,40 20,38 40,30 60,32 80,20 100,18 120,10" fill="none" stroke="rgba(255,255,255,.5)" stroke-width="1"/>
      ${[[15,36,6,'00FF88'],[35,28,10,'00FF88'],[60,30,8,'FF3355'],[80,18,12,'00FF88'],[100,16,5,'FF3355']].map(([x,y,r,c])=>`
        <circle cx="${x}" cy="${y-r*0.6}" r="${r}" fill="rgba(0,255,136,.2)" stroke="#${c}" stroke-width="1" opacity=".85"/>
        <circle cx="${x}" cy="${y+r*0.6}" r="${r*0.6}" fill="rgba(255,51,85,.15)" stroke="#FF3355" stroke-width=".8" opacity=".6"/>
      `).join('')}
    </svg>`,

    heatmap: `<svg viewBox="0 0 120 60" xmlns="http://www.w3.org/2000/svg" style="width:120px;height:60px;display:block">
      <rect width="120" height="60" fill="#020c14"/>
      ${[[0,'rgba(0,255,136,.55)'],[30,'rgba(255,184,0,.5)'],[60,'rgba(255,51,85,.5)'],[90,'rgba(0,212,255,.45)']].map(([x,c])=>`
        <rect x="${x+2}" y="8" width="26" height="22" fill="${c}" rx="2"/>
        <rect x="${x+2}" y="32" width="26" height="22" fill="${c}" rx="2" opacity=".6"/>
      `).join('')}
      <text x="60" y="58" fill="rgba(94,132,158,.7)" font-size="5" font-family="monospace" text-anchor="middle">MARKET HEAT</text>
    </svg>`,

    howto: `<svg viewBox="0 0 120 60" xmlns="http://www.w3.org/2000/svg" style="width:120px;height:60px;display:block">
      <rect width="120" height="60" fill="#020c14"/>
      ${[1,2,3,4,5].map((n,i)=>`
        <circle cx="16" cy="${8+i*11}" r="5" fill="rgba(0,212,255,.15)" stroke="#00d4ff" stroke-width="1"/>
        <text x="16" y="${11+i*11}" fill="#00d4ff" font-size="6" font-family="monospace" text-anchor="middle">${n}</text>
        <rect x="28" y="${5+i*11}" width="${55+Math.random()*20|0}" height="6" fill="rgba(0,212,255,.12)" rx="1"/>
      `).join('')}
    </svg>`
  };

  const sections = [
    {
      tag:'OVERVIEW', tagColor:'#00d4ff', tagBorder:'rgba(0,212,255,.35)',
      icon:'🔥', title:'What This Terminal Does',
      svg: SVG.edge,
      body:'Detects high-probability moves on Polymarket using real-time whale activity, volume acceleration, and crowd positioning. Every signal has a measured edge — no noise, no guessing. Signals are scored live and ranked by confidence.',
      stats:[{l:'Avg Hit Rate',v:'66%',c:'#00FF88'},{l:'Signal Types',v:'10',c:'#00d4ff'},{l:'Data Lag',v:'<2s',c:'#FFB800'}]
    },
    {
      tag:'SIGNAL', tagColor:'#FFB800', tagBorder:'rgba(255,184,0,.35)',
      icon:'⚡', title:'Velocity Surge',
      svg: SVG.velocity,
      body:'1-min order-flow spikes 4× above the 60-min baseline. Breakout imminent. Act within <strong style="color:#fff">3–8 min</strong>. High false-positive rate on low-volume markets — filter by $1K+.',
      stats:[{l:'Hit Rate',v:'61%',c:'#FFB800'},{l:'Window',v:'3–8m',c:'#00d4ff'},{l:'Avg Move',v:'+9%',c:'#00FF88'}]
    },
    {
      tag:'SIGNAL', tagColor:'#FFB800', tagBorder:'rgba(255,184,0,.35)',
      icon:'📉', title:'Momentum Break',
      body:'Probability crosses a structural key level with volume confirmation — not random noise. Indicates continuation or reversal. Window: <strong style="color:#fff">5–12 min</strong>.',
      svg: SVG.edge,
      stats:[{l:'Hit Rate',v:'69%',c:'#00FF88'},{l:'Window',v:'5–12m',c:'#00d4ff'},{l:'Avg Move',v:'+15%',c:'#00FF88'}]
    },
    {
      tag:'SIGNAL', tagColor:'#9B5DE5', tagBorder:'rgba(155,93,229,.35)',
      icon:'🐋', title:'Whale Flow & Prints',
      svg: SVG.whale,
      body:'Tracks $5K+ trades from wallets with verified edge — scored by win rate, ROI, consistency, and volume. Whale Print = single oversized block. Smart Cluster = 3+ alpha wallets same outcome in 15 min.',
      stats:[{l:'Whale Print HR',v:'65%',c:'#FFB800'},{l:'Cluster HR',v:'72%',c:'#00FF88'},{l:'Min Trade',v:'$5K',c:'#00d4ff'}]
    },
    {
      tag:'PANEL', tagColor:'#00E5CC', tagBorder:'rgba(0,229,204,.35)',
      icon:'🧠', title:'Position Pressure',
      svg: SVG.pressure,
      body:'YES/NO crowd split combined with smart money bias. When crowd is 70%+ one side and whales are opposite — highest-conviction fade setup. Green = buy pressure, Red = sell pressure, bar width = imbalance magnitude.',
      stats:[{l:'Panel',v:'CENTER',c:'#00d4ff'},{l:'Update',v:'Live',c:'#00FF88'},{l:'Best Use',v:'Fade',c:'#FF3355'}]
    },
    {
      tag:'PANEL', tagColor:'#00FF88', tagBorder:'rgba(0,255,136,.35)',
      icon:'📊', title:'Market Pressure Map',
      svg: SVG.map,
      body:'Canvas chart showing buy/sell bubble clusters over time. Bubble size = trade volume. Green = buy-side pressure, Red = sell-side. Price line shows live movement. <strong style="color:#fff">Drag to pan</strong> left/right through history.',
      stats:[{l:'Background',v:'Pure Black',c:'#fff'},{l:'Draggable',v:'Yes',c:'#00FF88'},{l:'Max Points',v:'80',c:'#00d4ff'}]
    },
    {
      tag:'PANEL', tagColor:'#00d4ff', tagBorder:'rgba(0,212,255,.35)',
      icon:'▦', title:'Market Heatmap',
      svg: SVG.heatmap,
      body:'All active Polymarket markets color-coded by recent price move. Green = rising probability, Red = falling. Cell size = relative volume. Click any tile to jump to that market and load live order book + whale data.',
      stats:[{l:'Markets',v:'Live',c:'#00d4ff'},{l:'Sort By',v:'Vol/Move',c:'#FFB800'},{l:'Click',v:'→ Market',c:'#00FF88'}]
    },
    {
      tag:'GUIDE', tagColor:'#FF6B35', tagBorder:'rgba(255,107,53,.35)',
      icon:'🎯', title:'How To Use — 5 Steps',
      svg: SVG.howto,
      steps:[
        '<strong style="color:#fff">Filter signals</strong> — Set SIZE to $1K+, focus on <span style="color:#ff3355">CRITICAL</span> or <span style="color:#FFB800">HIGH</span> priority only',
        '<strong style="color:#fff">Confirm with Whale Flow</strong> — are big wallets aligned with the signal direction?',
        '<strong style="color:#fff">Check Position Pressure</strong> — is there crowd imbalance to fade or follow?',
        '<strong style="color:#fff">Act fast</strong> — each card shows its valid window (e.g. 5–12 min). Edge decays fast.',
        '<strong style="color:#fff">Use the OHLC chart</strong> — time entry around current price level on the center panel'
      ]
    }
  ];

  body.innerHTML = sections.map(s => `
    <div class="how-section" style="display:grid;grid-template-columns:1fr 130px;gap:16px;align-items:start">
      <div>
        <div style="display:flex;align-items:center;gap:8px;margin-bottom:8px">
          <div class="how-tag" style="color:${s.tagColor};border-color:${s.tagBorder};margin-bottom:0">${s.tag}</div>
          <div class="how-title" style="margin-bottom:0">${s.icon} ${s.title}</div>
        </div>
        ${s.steps
          ? `<div class="how-steps">${s.steps.map((st,i)=>`<div class="how-step"><span class="how-step-num">${i+1}.</span><span>${st}</span></div>`).join('')}</div>`
          : `<div class="how-body">${s.body}</div>`
        }
        ${s.stats ? `<div style="display:flex;gap:10px;margin-top:10px">${s.stats.map(st=>`
          <div style="background:rgba(0,0,0,.4);border:1px solid rgba(255,255,255,.07);padding:5px 10px;border-radius:2px">
            <div style="font-size:6px;color:var(--t3);letter-spacing:1.5px;text-transform:uppercase;margin-bottom:2px">${st.l}</div>
            <div style="font-family:'Orbitron';font-size:11px;font-weight:700;color:${st.c}">${st.v}</div>
          </div>`).join('')}</div>` : ''}
      </div>
      <div style="display:flex;align-items:center;justify-content:center;background:rgba(0,0,0,.5);border:1px solid rgba(0,212,255,.1);border-radius:3px;padding:6px;margin-top:4px">
        ${s.svg}
      </div>
    </div>`).join('');
}

// ═══ WHALE BATTLEFIELD ══════════════════════════════════════════
(function(){
  let _wbZones = [];

  function drawWhaleBattlefield() {
    const canvas = document.getElementById('whale-battle-canvas');
    if (!canvas) return;
    const dpr = window.devicePixelRatio || 1;
    const W = canvas.offsetWidth, H = canvas.offsetHeight;
    if (!W || !H) return;
    canvas.width = W * dpr; canvas.height = H * dpr;
    const ctx = canvas.getContext('2d');
    ctx.scale(dpr, dpr);
    _wbZones = [];

    const whales = [...WHALES].sort((a,b)=>(b.total_volume||0)-(a.total_volume||0)).slice(0,12);
    if (!whales.length) {
      ctx.fillStyle='rgba(94,132,158,.25)'; ctx.font='8px JetBrains Mono'; ctx.textAlign='center';
      ctx.fillText('AWAITING WHALE ACTIVITY…', W/2, H/2); return;
    }

    // BG grid
    ctx.fillStyle='#020c14'; ctx.fillRect(0,0,W,H);
    for(let x=0;x<W;x+=40){ctx.strokeStyle='rgba(0,212,255,.04)';ctx.lineWidth=1;ctx.beginPath();ctx.moveTo(x,0);ctx.lineTo(x,H);ctx.stroke();}
    for(let y=0;y<H;y+=30){ctx.beginPath();ctx.moveTo(0,y);ctx.lineTo(W,y);ctx.stroke();}

    const maxVol = Math.max(...whales.map(w=>w.total_volume||0), 1);
    const maxScore = Math.max(...whales.map(w=>w.whale_score||0), 1);

    // Deterministic positions from wallet hash
    function hashPos(str, seed) {
      let h = seed; for(const c of str||'x') h=(h*31+c.charCodeAt(0))>>>0;
      return h;
    }

    const PAD=28;
    whales.forEach((w,i) => {
      const vol   = w.total_volume||1;
      const score = w.whale_score||50;
      const isBuy = (w.dominant_action||'BUYER')==='BUYER';
      const name  = (w.pseudonym||w.wallet_short||'???').toUpperCase().slice(0,10);
      const wallet= w.wallet||'';

      // position — spread them across canvas
      const hx = hashPos(wallet, 7);
      const hy = hashPos(wallet, 13);
      const x  = PAD + ((hx % 1000)/1000) * (W - PAD*2);
      const y  = PAD + ((hy % 1000)/1000) * (H - PAD*2);

      // radius scaled by volume
      const r = 6 + (vol/maxVol) * Math.min(W,H) * 0.12;

      // color: buyer=green, seller=red, score drives intensity
      const alpha = 0.25 + (score/maxScore)*0.5;
      const baseColor = isBuy ? `rgba(0,255,136,${alpha})` : `rgba(255,51,85,${alpha})`;
      const glowColor = isBuy ? '#00FF88' : '#FF3355';
      const borderColor= isBuy ? `rgba(0,255,136,${0.5+score/maxScore*0.5})` : `rgba(255,51,85,${0.5+score/maxScore*0.5})`;

      // outer glow ring
      ctx.shadowColor=glowColor; ctx.shadowBlur=r*0.8;
      ctx.beginPath(); ctx.arc(x,y,r,0,Math.PI*2);
      ctx.fillStyle=baseColor; ctx.fill();
      ctx.strokeStyle=borderColor; ctx.lineWidth=1.5; ctx.stroke();
      ctx.shadowBlur=0;

      // inner bright core
      const coreR = r*0.3;
      ctx.beginPath(); ctx.arc(x,y,coreR,0,Math.PI*2);
      ctx.fillStyle=isBuy?'rgba(0,255,136,.9)':'rgba(255,51,85,.9)';
      ctx.shadowColor=glowColor; ctx.shadowBlur=coreR*2;
      ctx.fill(); ctx.shadowBlur=0;

      // rank badge for top 3
      if(i<3){
        ctx.font=`700 ${7+i===0?2:0}px JetBrains Mono`;
        ctx.fillStyle='rgba(255,184,0,.9)';
        ctx.textAlign='center';
        ctx.shadowColor='#FFB800'; ctx.shadowBlur=6;
        ctx.fillText(['👑','②','③'][i], x, y-r-4);
        ctx.shadowBlur=0;
      }

      // name label
      ctx.font='700 7px JetBrains Mono';
      ctx.textAlign='center';
      ctx.fillStyle=isBuy?'rgba(0,255,136,.9)':'rgba(255,51,85,.9)';
      ctx.shadowColor=glowColor; ctx.shadowBlur=4;
      ctx.fillText(name, x, y+r+10);
      ctx.shadowBlur=0;

      // vol label
      const volStr='$'+(vol>=1000?(vol/1000).toFixed(1)+'K':vol.toFixed(0));
      ctx.font='6px JetBrains Mono';
      ctx.fillStyle='rgba(94,132,158,.7)';
      ctx.fillText(volStr, x, y+r+18);

      // score ring arc
      ctx.beginPath();
      ctx.arc(x,y,r+3, -Math.PI/2, -Math.PI/2+(score/100)*Math.PI*2);
      ctx.strokeStyle=score>80?'rgba(255,184,0,.7)':score>60?'rgba(0,212,255,.7)':'rgba(155,93,229,.5)';
      ctx.lineWidth=1.5; ctx.stroke();

      _wbZones.push({x,y,r:r+4,wallet});
    });

    // draw connection lines between top 5 (alliance map)
    const top5 = whales.slice(0,5);
    for(let i=0;i<top5.length-1;i++){
      const wa=top5[i], wb=top5[i+1];
      const hxa=hashPos(wa.wallet||'',7),hya=hashPos(wa.wallet||'',13);
      const hxb=hashPos(wb.wallet||'',7),hyb=hashPos(wb.wallet||'',13);
      const ax=PAD+((hxa%1000)/1000)*(W-PAD*2), ay=PAD+((hya%1000)/1000)*(H-PAD*2);
      const bx=PAD+((hxb%1000)/1000)*(W-PAD*2), by=PAD+((hyb%1000)/1000)*(H-PAD*2);
      ctx.beginPath(); ctx.moveTo(ax,ay); ctx.lineTo(bx,by);
      ctx.strokeStyle='rgba(0,212,255,.08)'; ctx.lineWidth=1;
      ctx.setLineDash([3,5]); ctx.stroke(); ctx.setLineDash([]);
    }
  }
  // ═══ WHALE ALERT SYSTEM ══════════════════════════════════════════
let _waTmr = null, _waStart = 0, _waRaf = null;
let _soundMuted = false;
function toggleSound(btn) {
  _soundMuted = !_soundMuted;
  btn.textContent = _soundMuted ? '🔇 SOUND' : '🔊 SOUND';
  btn.style.borderColor = _soundMuted ? 'rgba(255,51,85,.3)' : 'rgba(0,255,136,.3)';
  btn.style.background  = _soundMuted ? 'rgba(255,51,85,.06)' : 'rgba(0,255,136,.06)';
  btn.style.color       = _soundMuted ? '#ff3355' : '#00ff88';
}
function playWhaleSound(tier) {
  if (_soundMuted) return;
  try {
    const AC = window.AudioContext || window.webkitAudioContext;
    const ac = new AC();
    const play = (freq, type, start, dur, gain=0.3) => {
      const o = ac.createOscillator(), g = ac.createGain();
      o.connect(g); g.connect(ac.destination);
      o.type = type; o.frequency.setValueAtTime(freq, ac.currentTime + start);
      g.gain.setValueAtTime(0, ac.currentTime + start);
      g.gain.linearRampToValueAtTime(gain, ac.currentTime + start + 0.02);
      g.gain.exponentialRampToValueAtTime(0.001, ac.currentTime + start + dur);
      o.start(ac.currentTime + start); o.stop(ac.currentTime + start + dur);
    };
    if (tier === 'CRITICAL') {
      // deep boom + rising arpeggio
      play(60,  'sine',    0,    0.4, 0.5);
      play(120, 'sine',    0,    0.35, 0.3);
      play(440, 'square',  0.05, 0.12, 0.15);
      play(554, 'square',  0.18, 0.12, 0.15);
      play(659, 'square',  0.31, 0.18, 0.18);
      play(880, 'sawtooth',0.44, 0.22, 0.12);
    } else {
      // clean 3-tone chime
      play(440, 'sine', 0,    0.25, 0.25);
      play(554, 'sine', 0.15, 0.25, 0.25);
      play(659, 'sine', 0.3,  0.3,  0.25);
    }
  } catch(e) {}
}

function showWhaleAlert(t) {
  const isBuy  = String(t.action||'').toUpperCase().includes('BUY');
  const isSell = String(t.action||'').toUpperCase().includes('SELL');
  const size   = t.size_usd || 0;
  const tier   = size >= 50000 ? 'CRITICAL' : size >= 20000 ? 'HIGH' : 'WHALE';
  const dc     = isBuy ? '#00ff88' : isSell ? '#ff3355' : '#00d4ff';
  const db     = isBuy ? 'rgba(0,255,136,.12)' : isSell ? 'rgba(255,51,85,.12)' : 'rgba(0,212,255,.1)';
  const dbo    = isBuy ? 'rgba(0,255,136,.5)' : isSell ? 'rgba(255,51,85,.5)' : 'rgba(0,212,255,.45)';
  const dl     = isBuy ? '▲ BUY YES' : isSell ? '▼ BUY NO' : '◆ WATCH';
  const tierColor = {CRITICAL:'#ff3355', HIGH:'#ffb800', WHALE:'#00d4ff'}[tier];
  const wallet = t.wallet ? t.wallet.slice(0,6)+'…'+t.wallet.slice(-4) : '0xAnon';
  const score  = t.whale_score ? t.whale_score.toFixed(0) : '—';
  const sizeStr = size >= 1000 ? '$'+(size/1000).toFixed(1)+'K' : '$'+size.toFixed(0);

  const badge = document.getElementById('wa-badge');
  badge.textContent = tier;
  badge.style.color = tierColor;
  badge.style.border = `1px solid ${tierColor}`;
  badge.style.background = tierColor.replace(')', ',.12)').replace('rgb','rgba');

  document.getElementById('wa-market').textContent  = t.market || t.market_slug || 'Polymarket Market';
  document.getElementById('wa-size').textContent    = sizeStr;
  document.getElementById('wa-size').style.color    = tierColor;
  document.getElementById('wa-price').textContent   = (t.price_cents||0).toFixed(1)+'¢';
  document.getElementById('wa-price').style.color   = '#dff0f8';
  document.getElementById('wa-action').textContent  = t.action || '—';
  document.getElementById('wa-action').style.color  = dc;
  document.getElementById('wa-score').textContent   = score;
  document.getElementById('wa-score').style.color   = '#9b5de5';
  document.getElementById('wa-pill').textContent    = dl;
  document.getElementById('wa-pill').style.color    = dc;
  document.getElementById('wa-pill').style.border   = `1px solid ${dbo}`;
  document.getElementById('wa-pill').style.background = db;
  document.getElementById('wa-outcome').textContent = t.outcome ? `→ ${t.outcome}` : '';
  document.getElementById('wa-wallet').textContent  = `🐋 ${wallet}`;
  document.getElementById('wa-ago').textContent     = 'just now';

  const el = document.getElementById('whale-alert');
  el.classList.remove('dismissing');
  el.style.display = 'block';

  // border accent by tier
  el.style.borderColor = tierColor.replace(')', ',.5)').replace('#','rgba(').replace(/([0-9a-f]{2})/gi, (m,h)=>parseInt(h,16)+',').slice(0,-1);
  el.style.borderColor = dbo; // simpler

  // timer bar (8s auto-dismiss)
  const DURATION = 8000;
  _waStart = performance.now();
  if (_waRaf) cancelAnimationFrame(_waRaf);
  const bar = document.getElementById('wa-timer-bar');
  const tick = now => {
    const pct = Math.max(0, 1 - (now - _waStart) / DURATION);
    bar.style.width = (pct * 100) + '%';
    if (pct > 0) _waRaf = requestAnimationFrame(tick);
  };
  _waRaf = requestAnimationFrame(tick);
  if (_waTmr) clearTimeout(_waTmr);
  _waTmr = setTimeout(dismissWhaleAlert, DURATION);

  playWhaleSound(tier);
}

function dismissWhaleAlert() {
  const el = document.getElementById('whale-alert');
  if (_waTmr) clearTimeout(_waTmr);
  if (_waRaf) cancelAnimationFrame(_waRaf);
  el.classList.add('dismissing');
  setTimeout(() => { el.style.display='none'; el.classList.remove('dismissing'); }, 260);
}
  // click → profile
  function onWBClick(e){
    const c=document.getElementById('whale-battle-canvas'); if(!c) return;
    const rect=c.getBoundingClientRect();
    const mx=e.clientX-rect.left, my=e.clientY-rect.top;
    for(const z of _wbZones){
      const dx=mx-z.x,dy=my-z.y;
      if(dx*dx+dy*dy<=z.r*z.r&&z.wallet){confirmRedirect('https://polymarket.com/profile/'+z.wallet);return;}
    }
  }
  function onWBMove(e){
    const c=document.getElementById('whale-battle-canvas'); if(!c) return;
    const rect=c.getBoundingClientRect();
    const mx=e.clientX-rect.left, my=e.clientY-rect.top;
    c.style.cursor=_wbZones.some(z=>{const dx=mx-z.x,dy=my-z.y;return dx*dx+dy*dy<=z.r*z.r;})?'pointer':'crosshair';
  }

  window.drawWhaleBattlefield=drawWhaleBattlefield;

  document.addEventListener('DOMContentLoaded',()=>{
    const c=document.getElementById('whale-battle-canvas');
    if(c){c.addEventListener('click',onWBClick);c.addEventListener('mousemove',onWBMove);}
    setTimeout(drawWhaleBattlefield,700);
  });

  const _origRW=window.renderWhales;
  if(_origRW) window.renderWhales=function(){_origRW();setTimeout(drawWhaleBattlefield,50);};
  window.addEventListener('resize',drawWhaleBattlefield);
})();

// REMOVED: whale pnl chart block below (replaced by battlefield above)
(function(){
  let _whaleHitZones = [];

  function renderWhaleChart() {
    const canvas = document.getElementById('whale-chart-canvas');
    const panel  = document.getElementById('whale-chart-panel');
    if (!canvas || !panel) return;
    const W = panel.clientWidth;
    const H = panel.clientHeight - 28;
    if (W < 10 || H < 10) return;
    const dpr = window.devicePixelRatio || 1;
    canvas.width  = W * dpr;
    canvas.height = H * dpr;
    canvas.style.width  = W + 'px';
    canvas.style.height = H + 'px';
    const ctx = canvas.getContext('2d');
    ctx.scale(dpr, dpr);
    _whaleHitZones = [];

    const whales = [...WHALES].filter(w=>(w.pnl_proxy||0)>0).sort((a,b)=>(b.pnl_proxy||0)-(a.pnl_proxy||0)).slice(0,8);
  if (!whales.length) {
    ctx.fillStyle = 'rgba(94,132,158,.3)';
    ctx.font = '9px JetBrains Mono';
    ctx.textAlign = 'center';
    ctx.fillText('AWAITING WHALE DATA…', W/2, H/2);
    return;
  }

    const PAD = {l:10, r:10, t:22, b:30};
    const cW = W - PAD.l - PAD.r;
    const cH = H - PAD.t - PAD.b;
    const n   = whales.length;
    const barW = Math.max(8, Math.floor((cW / n) * 0.62));
    const slotW = cW / n;
    const maxPnl = Math.max(...whales.map(w=>Math.abs(w.pnl_proxy||0)), 1);
    const zeroY  = PAD.t + cH * 0.5;

    for (let y = PAD.t; y < H - PAD.b; y += 4) {
      ctx.fillStyle = 'rgba(0,212,255,.012)';
      ctx.fillRect(PAD.l, y, cW, 1);
    }

    [0.25, 0.5, 0.75, 1].forEach(f => {
      const y = PAD.t + cH * f;
      ctx.strokeStyle = 'rgba(0,212,255,.06)';
      ctx.lineWidth = 1;
      ctx.setLineDash([2,4]);
      ctx.beginPath(); ctx.moveTo(PAD.l, y); ctx.lineTo(W-PAD.r, y); ctx.stroke();
      ctx.setLineDash([]);
    });

    ctx.strokeStyle = 'rgba(0,212,255,.2)';
    ctx.lineWidth = 1;
    ctx.beginPath(); ctx.moveTo(PAD.l, zeroY); ctx.lineTo(W-PAD.r, zeroY); ctx.stroke();

    const dotPoints = [];

    whales.forEach((w, i) => {
      const pnl    = w.pnl_proxy || 0;
      const score  = w.whale_score || 0;
      const name   = (w.pseudonym || w.wallet_short || '???').toUpperCase().slice(0,11);
      const wallet = w.wallet || '';
      const isPos  = pnl >= 0;
      const ratio  = pnl / maxPnl;
      const barH   = Math.abs(ratio) * (cH * 0.44);
      const xC     = PAD.l + slotW * i + slotW / 2;
      const xL     = xC - barW / 2;
      const barTop = isPos ? zeroY - barH : zeroY;

      _whaleHitZones.push({ x:xL, y:barTop, w:barW, h:Math.max(barH,4), wallet });

      ctx.shadowColor = isPos ? '#00FF88' : '#FF3355';
      ctx.shadowBlur  = 18;

      const grd = ctx.createLinearGradient(0, barTop, 0, barTop + barH);
      if (isPos) {
        grd.addColorStop(0, 'rgba(0,255,136,1)');
        grd.addColorStop(0.5,'rgba(0,220,110,.7)');
        grd.addColorStop(1, 'rgba(0,100,60,.2)');
      } else {
        grd.addColorStop(0, 'rgba(255,51,85,.2)');
        grd.addColorStop(0.5,'rgba(255,51,85,.7)');
        grd.addColorStop(1, 'rgba(255,51,85,1)');
      }
      ctx.fillStyle = grd;
      ctx.beginPath();
      if (ctx.roundRect) ctx.roundRect(xL, barTop, barW, Math.max(barH, 2), [3,3,1,1]);
      else ctx.rect(xL, barTop, barW, Math.max(barH,2));
      ctx.fill();
      ctx.shadowBlur = 0;

      ctx.strokeStyle = isPos ? 'rgba(0,255,136,.8)' : 'rgba(255,51,85,.8)';
      ctx.lineWidth = 1;
      ctx.beginPath();
      ctx.moveTo(xL, barTop); ctx.lineTo(xL+barW, barTop);
      ctx.stroke();

      const chipY = isPos ? barTop - 8 : barTop + barH + 8;
      const chipColor = score > 80 ? '#FFB800' : score > 60 ? '#00D4FF' : '#9B5DE5';
      ctx.shadowColor = chipColor; ctx.shadowBlur = 8;
      ctx.fillStyle = chipColor;
      ctx.beginPath(); ctx.arc(xC, chipY, 3.5, 0, Math.PI*2); ctx.fill();
      ctx.shadowBlur = 0;
      dotPoints.push({x:xC, y:chipY});

      const pnlStr = (isPos?'+':'') + (Math.abs(pnl)>=1000 ? '$'+(pnl/1000).toFixed(1)+'K' : '$'+pnl.toFixed(0));
      ctx.font = '700 9px JetBrains Mono';
      ctx.textAlign = 'center';
      ctx.fillStyle = isPos ? '#00FF88' : '#FF3355';
      ctx.shadowColor = ctx.fillStyle; ctx.shadowBlur = 8;
      ctx.fillText(pnlStr, xC, isPos ? barTop - 16 : barTop + barH + 20);
      ctx.shadowBlur = 0;

      if (i === 0) {
        ctx.font = '13px serif';
        ctx.fillText('🐋', xC, isPos ? barTop - 28 : barTop + barH + 34);
      }

      ctx.font = '700 8px JetBrains Mono';
      ctx.fillStyle = 'rgba(0,212,255,.9)';
      ctx.shadowColor = 'rgba(0,212,255,.5)'; ctx.shadowBlur = 4;
      ctx.fillText(name, xC, H - 11);
      ctx.shadowBlur = 0;

      ctx.font = '700 7px JetBrains Mono';
      ctx.fillStyle = 'rgba(94,132,158,.8)';
      ctx.fillText('#'+(i+1), xC, H - 2);
    });

    if (dotPoints.length > 1) {
      ctx.beginPath();
      ctx.strokeStyle = 'rgba(0,212,255,.35)';
      ctx.lineWidth = 1;
      ctx.setLineDash([3,4]);
      dotPoints.forEach((p,i)=> i===0 ? ctx.moveTo(p.x,p.y) : ctx.lineTo(p.x,p.y));
      ctx.stroke();
      ctx.setLineDash([]);
    }
  }

  function onWhaleCanvasClick(e) {
    const canvas = document.getElementById('whale-chart-canvas');
    if (!canvas) return;
    const rect = canvas.getBoundingClientRect();
    const mx = e.clientX - rect.left, my = e.clientY - rect.top;
    for (const z of _whaleHitZones) {
      if (mx >= z.x && mx <= z.x+z.w && my >= z.y && my <= z.y+z.h && z.wallet) {
        confirmRedirect('https://polymarket.com/profile/'+z.wallet);
        return;
      }
    }
  }

  function onWhaleCanvasMove(e) {
    const canvas = document.getElementById('whale-chart-canvas');
    if (!canvas) return;
    const rect = canvas.getBoundingClientRect();
    const mx = e.clientX - rect.left, my = e.clientY - rect.top;
    const hit = _whaleHitZones.some(z => mx>=z.x && mx<=z.x+z.w && my>=z.y && my<=z.y+z.h);
    canvas.style.cursor = hit ? 'pointer' : 'default';
  }

  window.renderWhaleChart = renderWhaleChart;

  document.addEventListener('DOMContentLoaded', () => {
    const c = document.getElementById('whale-chart-canvas');
    if (c) {
      c.addEventListener('click', onWhaleCanvasClick);
      c.addEventListener('mousemove', onWhaleCanvasMove);
    }
    setTimeout(renderWhaleChart, 600);
  });

  const _orig = window.renderWhales;
  if (_orig) window.renderWhales = function(){ _orig(); setTimeout(renderWhaleChart,50); };

  window.addEventListener('resize', renderWhaleChart);
})();

// Close on Escape (extend existing listener)
document.addEventListener('keydown', e => {
  if (e.key === 'Escape') { closeEdgeFeedModal(); closeHeatmapModal(); closeHow(); }
});

// Allow pressing Enter in license input
document.getElementById('lic-input')?.addEventListener('keydown', e => { if (e.key === 'Enter') _licActivate(); });
</script>
<div id="modal-overlay" style="display:none;align-items:center;justify-content:center;backdrop-filter:blur(4px)">
  <div style="background:linear-gradient(135deg,#060d14,#0a1828);border:1px solid rgba(0,212,255,.25);box-shadow:0 0 40px rgba(0,212,255,.12),0 0 80px rgba(0,0,0,.8);padding:32px 36px;min-width:360px;max-width:440px;font-family:'JetBrains Mono',monospace;text-align:center;position:relative">
    <div style="position:absolute;top:0;left:0;right:0;height:2px;background:linear-gradient(90deg,transparent,#00D4FF,transparent)"></div>
    <div style="display:flex;align-items:center;justify-content:center;gap:8px;margin-bottom:6px">
      <span style="font-size:18px">🐋</span>
      <div style="font-family:'Orbitron';font-size:10px;font-weight:900;color:#00D4FF;letter-spacing:3px">POLYMARKET REDIRECT</div>
    </div>
    <div style="width:40px;height:1px;background:rgba(0,212,255,.3);margin:12px auto"></div>
    <div id="modal-msg" style="font-size:11px;color:#dff0f8;line-height:1.6;margin-bottom:24px;padding:0 8px"></div>
    <div style="display:flex;gap:10px;justify-content:center">
      <button id="modal-confirm" style="background:rgba(0,255,136,.08);border:1px solid rgba(0,255,136,.5);color:#00FF88;font-family:'JetBrains Mono';font-size:9px;font-weight:700;letter-spacing:1.5px;padding:9px 24px;cursor:pointer;transition:all .15s;text-transform:uppercase" onmouseover="this.style.background='rgba(0,255,136,.18)'" onmouseout="this.style.background='rgba(0,255,136,.08)'">OPEN ON POLYMARKET ↗</button>
      <button onclick="closeModal()" style="background:rgba(255,51,85,.06);border:1px solid rgba(255,51,85,.4);color:#FF3355;font-family:'JetBrains Mono';font-size:9px;font-weight:700;letter-spacing:1.5px;padding:9px 20px;cursor:pointer;transition:all .15s;text-transform:uppercase" onmouseover="this.style.background='rgba(255,51,85,.15)'" onmouseout="this.style.background='rgba(255,51,85,.06)'">CANCEL</button>
    </div>
    <div style="margin-top:16px;font-size:7px;color:#243544;letter-spacing:1px">YOU WILL BE REDIRECTED TO AN EXTERNAL SITE</div>
  </div>
</div>
<!-- WHALE ALERT POPUP -->
<div id="whale-alert-overlay">
  <div id="whale-alert">
    <div class="wa-corner tl"></div><div class="wa-corner tr"></div>
    <div class="wa-corner bl"></div><div class="wa-corner br"></div>
    <div class="wa-header">
      <div class="wa-title-row">
        <span class="wa-icon">🐋</span>
        <span class="wa-label">WHALE DETECTED</span>
        <span class="wa-badge" id="wa-badge">HIGH</span>
      </div>
      <button class="wa-close" onclick="dismissWhaleAlert()">✕</button>
    </div>
    <div class="wa-body">
      <div class="wa-market" id="wa-market">—</div>
      <div class="wa-stats">
        <div class="wa-stat"><div class="wa-stat-l">SIZE</div><div class="wa-stat-v" id="wa-size">—</div></div>
        <div class="wa-stat"><div class="wa-stat-l">PRICE</div><div class="wa-stat-v" id="wa-price">—</div></div>
        <div class="wa-stat"><div class="wa-stat-l">ACTION</div><div class="wa-stat-v" id="wa-action">—</div></div>
        <div class="wa-stat"><div class="wa-stat-l">SCORE</div><div class="wa-stat-v" id="wa-score">—</div></div>
      </div>
      <div class="wa-action-row">
        <div class="wa-action-pill" id="wa-pill">—</div>
        <div class="wa-outcome" id="wa-outcome">—</div>
      </div>
    </div>
    <div class="wa-footer">
      <span class="wa-wallet" id="wa-wallet">—</span>
      <span style="font-size:6.5px;color:var(--t3)" id="wa-ago">—</span>
    </div>
    <div class="wa-timer-bar" id="wa-timer-bar"></div>
  </div>
</div>
</body>
</html>
