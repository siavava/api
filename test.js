var luxon = ((e) => {
  function L(e, t) {
    for (var n = 0; n < t.length; n++) {
      var r = t[n];
      ((r.enumerable = r.enumerable || !1),
        (r.configurable = !0),
        "value" in r && (r.writable = !0),
        Object.defineProperty(
          e,
          ((e) =>
            "symbol" ==
            typeof (e = ((e, t) => {
              if ("object" != typeof e || null === e) return e;
              var n = e[Symbol.toPrimitive];
              if (void 0 === n) return ("string" === t ? String : Number)(e);
              if ("object" != typeof (n = n.call(e, t || "default"))) return n;
              throw new TypeError(
                "@@toPrimitive must return a primitive value.",
              );
            })(e, "string"))
              ? e
              : String(e))(r.key),
          r,
        ));
    }
  }
  function i(e, t, n) {
    (t && L(e.prototype, t),
      n && L(e, n),
      Object.defineProperty(e, "prototype", { writable: !1 }));
  }
  function l() {
    return (l = Object.assign
      ? Object.assign.bind()
      : function (e) {
          for (var t = 1; t < arguments.length; t++) {
            var n,
              r = arguments[t];
            for (n in r)
              Object.prototype.hasOwnProperty.call(r, n) && (e[n] = r[n]);
          }
          return e;
        }).apply(this, arguments);
  }
  function o(e, t) {
    ((e.prototype = Object.create(t.prototype)),
      z((e.prototype.constructor = e), t));
  }
  function j(e) {
    return (j = Object.setPrototypeOf
      ? Object.getPrototypeOf.bind()
      : function (e) {
          return e.__proto__ || Object.getPrototypeOf(e);
        })(e);
  }
  function z(e, t) {
    return (z = Object.setPrototypeOf
      ? Object.setPrototypeOf.bind()
      : function (e, t) {
          return ((e.__proto__ = t), e);
        })(e, t);
  }
  function A(e, t, n) {
    return (A = (() => {
      if (
        "undefined" != typeof Reflect &&
        Reflect.construct &&
        !Reflect.construct.sham
      ) {
        if ("function" == typeof Proxy) return 1;
        try {
          return (
            Boolean.prototype.valueOf.call(
              Reflect.construct(Boolean, [], function () {}),
            ),
            1
          );
        } catch (e) {}
      }
    })()
      ? Reflect.construct.bind()
      : function (e, t, n) {
          var r = [null];
          r.push.apply(r, t);
          t = new (Function.bind.apply(e, r))();
          return (n && z(t, n.prototype), t);
        }).apply(null, arguments);
  }
  function q(e) {
    var n = "function" == typeof Map ? new Map() : void 0;
    return (function (e) {
      if (
        null === e ||
        -1 === Function.toString.call(e).indexOf("[native code]")
      )
        return e;
      if ("function" != typeof e)
        throw new TypeError(
          "Super expression must either be null or a function",
        );
      if (void 0 !== n) {
        if (n.has(e)) return n.get(e);
        n.set(e, t);
      }
      function t() {
        return A(e, arguments, j(this).constructor);
      }
      return (
        (t.prototype = Object.create(e.prototype, {
          constructor: {
            value: t,
            enumerable: !1,
            writable: !0,
            configurable: !0,
          },
        })),
        z(t, e)
      );
    })(e);
  }
  function _(e, t) {
    if (null == e) return {};
    for (var n, r = {}, i = Object.keys(e), o = 0; o < i.length; o++)
      ((n = i[o]), 0 <= t.indexOf(n) || (r[n] = e[n]));
    return r;
  }
  function U(e, t) {
    (null == t || t > e.length) && (t = e.length);
    for (var n = 0, r = new Array(t); n < t; n++) r[n] = e[n];
    return r;
  }
  function R(e, t) {
    var n,
      r =
        ("undefined" != typeof Symbol && e[Symbol.iterator]) || e["@@iterator"];
    if (r) return (r = r.call(e)).next.bind(r);
    if (
      Array.isArray(e) ||
      (r = ((e, t) => {
        var n;
        if (e)
          return "string" == typeof e
            ? U(e, t)
            : "Map" ===
                  (n =
                    "Object" ===
                      (n = Object.prototype.toString.call(e).slice(8, -1)) &&
                    e.constructor
                      ? e.constructor.name
                      : n) || "Set" === n
              ? Array.from(e)
              : "Arguments" === n ||
                  /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n)
                ? U(e, t)
                : void 0;
      })(e)) ||
      (t && e && "number" == typeof e.length)
    )
      return (
        r && (e = r),
        (n = 0),
        function () {
          return n >= e.length ? { done: !0 } : { done: !1, value: e[n++] };
        }
      );
    throw new TypeError(
      "Invalid attempt to iterate non-iterable instance.\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.",
    );
  }
  var P = ((t) => {
      function e(e) {
        return t.call(this, "Invalid DateTime: " + e.toMessage()) || this;
      }
      return (o(e, t), e);
    })(
      (k = ((e) => {
        function t() {
          return e.apply(this, arguments) || this;
        }
        return (o(t, e), t);
      })(q(Error))),
    ),
    Y = ((t) => {
      function e(e) {
        return t.call(this, "Invalid Interval: " + e.toMessage()) || this;
      }
      return (o(e, t), e);
    })(k),
    H = ((t) => {
      function e(e) {
        return t.call(this, "Invalid Duration: " + e.toMessage()) || this;
      }
      return (o(e, t), e);
    })(k),
    w = ((e) => {
      function t() {
        return e.apply(this, arguments) || this;
      }
      return (o(t, e), t);
    })(k),
    J = ((t) => {
      function e(e) {
        return t.call(this, "Invalid unit " + e) || this;
      }
      return (o(e, t), e);
    })(k),
    u = ((e) => {
      function t() {
        return e.apply(this, arguments) || this;
      }
      return (o(t, e), t);
    })(k),
    n = ((e) => {
      function t() {
        return e.call(this, "Zone is an abstract class") || this;
      }
      return (o(t, e), t);
    })(k),
    G = { year: (k = "numeric"), month: k, day: k },
    $ = { year: k, month: (t = "short"), day: k },
    B = { year: k, month: t, day: k, weekday: t },
    Q = { year: k, month: (M = "long"), day: k },
    K = { year: k, month: M, day: k, weekday: M },
    X = { hour: k, minute: k },
    ee = { hour: k, minute: k, second: k },
    te = { hour: k, minute: k, second: k, timeZoneName: t },
    ne = { hour: k, minute: k, second: k, timeZoneName: M },
    re = { hour: k, minute: k, hourCycle: "h23" },
    ie = { hour: k, minute: k, second: k, hourCycle: "h23" },
    oe = { hour: k, minute: k, second: k, hourCycle: "h23", timeZoneName: t },
    ae = { hour: k, minute: k, second: k, hourCycle: "h23", timeZoneName: M },
    se = { year: k, month: k, day: k, hour: k, minute: k },
    ue = { year: k, month: k, day: k, hour: k, minute: k, second: k },
    le = { year: k, month: t, day: k, hour: k, minute: k },
    ce = { year: k, month: t, day: k, hour: k, minute: k, second: k },
    fe = { year: k, month: t, day: k, weekday: t, hour: k, minute: k },
    de = { year: k, month: M, day: k, hour: k, minute: k, timeZoneName: t },
    he = {
      year: k,
      month: M,
      day: k,
      hour: k,
      minute: k,
      second: k,
      timeZoneName: t,
    },
    me = {
      year: k,
      month: M,
      day: k,
      weekday: M,
      hour: k,
      minute: k,
      timeZoneName: M,
    },
    ye = {
      year: k,
      month: M,
      day: k,
      weekday: M,
      hour: k,
      minute: k,
      second: k,
      timeZoneName: M,
    },
    r = (() => {
      function e() {}
      var t = e.prototype;
      return (
        (t.offsetName = function (e, t) {
          throw new n();
        }),
        (t.formatOffset = function (e, t) {
          throw new n();
        }),
        (t.offset = function (e) {
          throw new n();
        }),
        (t.equals = function (e) {
          throw new n();
        }),
        i(e, [
          {
            key: "type",
            get: function () {
              throw new n();
            },
          },
          {
            key: "name",
            get: function () {
              throw new n();
            },
          },
          {
            key: "ianaName",
            get: function () {
              return this.name;
            },
          },
          {
            key: "isUniversal",
            get: function () {
              throw new n();
            },
          },
          {
            key: "isValid",
            get: function () {
              throw new n();
            },
          },
        ]),
        e
      );
    })(),
    ve = null,
    ge = ((e) => {
      function t() {
        return e.apply(this, arguments) || this;
      }
      o(t, e);
      var n = t.prototype;
      return (
        (n.offsetName = function (e, t) {
          return bt(e, t.format, t.locale);
        }),
        (n.formatOffset = function (e, t) {
          return Nt(this.offset(e), t);
        }),
        (n.offset = function (e) {
          return -new Date(e).getTimezoneOffset();
        }),
        (n.equals = function (e) {
          return "system" === e.type;
        }),
        i(
          t,
          [
            {
              key: "type",
              get: function () {
                return "system";
              },
            },
            {
              key: "name",
              get: function () {
                return new Intl.DateTimeFormat().resolvedOptions().timeZone;
              },
            },
            {
              key: "isUniversal",
              get: function () {
                return !1;
              },
            },
            {
              key: "isValid",
              get: function () {
                return !0;
              },
            },
          ],
          [
            {
              key: "instance",
              get: function () {
                return (ve = null === ve ? new t() : ve);
              },
            },
          ],
        ),
        t
      );
    })(r),
    pe = {},
    ke = { year: 0, month: 1, day: 2, era: 3, hour: 4, minute: 5, second: 6 },
    we = {},
    a = ((n) => {
      function r(e) {
        var t = n.call(this) || this;
        return ((t.zoneName = e), (t.valid = r.isValidZone(e)), t);
      }
      (o(r, n),
        (r.create = function (e) {
          return (we[e] || (we[e] = new r(e)), we[e]);
        }),
        (r.resetCache = function () {
          ((we = {}), (pe = {}));
        }),
        (r.isValidSpecifier = function (e) {
          return this.isValidZone(e);
        }),
        (r.isValidZone = function (e) {
          if (!e) return !1;
          try {
            return (
              new Intl.DateTimeFormat("en-US", { timeZone: e }).format(),
              !0
            );
          } catch (e) {
            return !1;
          }
        }));
      var e = r.prototype;
      return (
        (e.offsetName = function (e, t) {
          return bt(e, t.format, t.locale, this.name);
        }),
        (e.formatOffset = function (e, t) {
          return Nt(this.offset(e), t);
        }),
        (e.offset = function (e) {
          var t,
            n,
            r,
            i,
            o,
            a,
            s,
            u,
            e = new Date(e);
          return isNaN(e)
            ? NaN
            : ((o = this.name),
              pe[o] ||
                (pe[o] = new Intl.DateTimeFormat("en-US", {
                  hour12: !1,
                  timeZone: o,
                  year: "numeric",
                  month: "2-digit",
                  day: "2-digit",
                  hour: "2-digit",
                  minute: "2-digit",
                  second: "2-digit",
                  era: "short",
                })),
              (a = (o = (o = pe[o]).formatToParts
                ? ((e, t) => {
                    for (
                      var n = e.formatToParts(t), r = [], i = 0;
                      i < n.length;
                      i++
                    ) {
                      var o = n[i],
                        a = o.type,
                        o = o.value,
                        s = ke[a];
                      "era" === a
                        ? (r[s] = o)
                        : N(s) || (r[s] = parseInt(o, 10));
                    }
                    return r;
                  })(o, e)
                : ((a = e),
                  (o = (o = o).format(a).replace(/\u200E/g, "")),
                  (a = /(\d+)\/(\d+)\/(\d+) (AD|BC),? (\d+):(\d+):(\d+)/.exec(
                    o,
                  )),
                  (o = a[1]),
                  [a[3], o, a[2], a[4], a[5], a[6], a[7]]))[0]),
              (t = o[1]),
              (n = o[2]),
              (r = o[5]),
              (i = o[6]),
              (s = 24 === (s = o[4]) ? 0 : s),
              (u = (e = +e) % 1e3),
              (gt({
                year: (a = "BC" === o[3] ? 1 - Math.abs(a) : a),
                month: t,
                day: n,
                hour: s,
                minute: r,
                second: i,
                millisecond: 0,
              }) -
                (e -= 0 <= u ? u : 1e3 + u)) /
                6e4);
        }),
        (e.equals = function (e) {
          return "iana" === e.type && e.name === this.name;
        }),
        i(r, [
          {
            key: "type",
            get: function () {
              return "iana";
            },
          },
          {
            key: "name",
            get: function () {
              return this.zoneName;
            },
          },
          {
            key: "isUniversal",
            get: function () {
              return !1;
            },
          },
          {
            key: "isValid",
            get: function () {
              return this.valid;
            },
          },
        ]),
        r
      );
    })(r),
    be = ["base"],
    Se = ["padTo", "floor"],
    Oe = {},
    Te = {};
  function Ne(e, t) {
    void 0 === t && (t = {});
    var n = JSON.stringify([e, t]),
      r = Te[n];
    return (r || ((r = new Intl.DateTimeFormat(e, t)), (Te[n] = r)), r);
  }
  var De = {},
    Me = {},
    Ie = null,
    Ve = {};
  function Ee(e, t, n, r) {
    e = e.listingMode();
    return "error" === e ? null : ("en" === e ? n : r)(t);
  }
  var xe = (() => {
      function e(e, t, n) {
        ((this.padTo = n.padTo || 0), (this.floor = n.floor || !1));
        var r = _(n, Se);
        (!t || 0 < Object.keys(r).length) &&
          ((t = l({ useGrouping: !1 }, n)),
          0 < n.padTo && (t.minimumIntegerDigits = n.padTo),
          (this.inf =
            ((r = e),
            void 0 === (n = t) && (n = {}),
            (e = JSON.stringify([r, n])),
            (t = De[e]) || ((t = new Intl.NumberFormat(r, n)), (De[e] = t)),
            t)));
      }
      return (
        (e.prototype.format = function (e) {
          var t;
          return this.inf
            ? ((t = this.floor ? Math.floor(e) : e), this.inf.format(t))
            : h(this.floor ? Math.floor(e) : ht(e, 3), this.padTo);
        }),
        e
      );
    })(),
    Fe = (() => {
      function e(e, t, n) {
        this.opts = n;
        var n = (this.originalZone = void 0),
          r =
            (this.opts.timeZone
              ? (this.dt = e)
              : "fixed" === e.zone.type
                ? ((r =
                    0 <= (r = (e.offset / 60) * -1)
                      ? "Etc/GMT+" + r
                      : "Etc/GMT" + r),
                  0 !== e.offset && a.create(r).valid
                    ? ((n = r), (this.dt = e))
                    : ((n = "UTC"),
                      (this.dt =
                        0 === e.offset
                          ? e
                          : e.setZone("UTC").plus({ minutes: e.offset })),
                      (this.originalZone = e.zone)))
                : "system" === e.zone.type
                  ? (this.dt = e)
                  : "iana" === e.zone.type
                    ? (n = (this.dt = e).zone.name)
                    : ((this.dt = e
                        .setZone((n = "UTC"))
                        .plus({ minutes: e.offset })),
                      (this.originalZone = e.zone)),
            l({}, this.opts));
        ((r.timeZone = r.timeZone || n), (this.dtf = Ne(t, r)));
      }
      var t = e.prototype;
      return (
        (t.format = function () {
          return this.originalZone
            ? this.formatToParts()
                .map(function (e) {
                  return e.value;
                })
                .join("")
            : this.dtf.format(this.dt.toJSDate());
        }),
        (t.formatToParts = function () {
          var t = this,
            e = this.dtf.formatToParts(this.dt.toJSDate());
          return this.originalZone
            ? e.map(function (e) {
                return "timeZoneName" === e.type
                  ? l({}, e, {
                      value: t.originalZone.offsetName(t.dt.ts, {
                        locale: t.dt.locale,
                        format: t.opts.timeZoneName,
                      }),
                    })
                  : e;
              })
            : e;
        }),
        (t.resolvedOptions = function () {
          return this.dtf.resolvedOptions();
        }),
        e
      );
    })(),
    Ce = (() => {
      function e(e, t, n) {
        var r;
        ((this.opts = l({ style: "long" }, n)),
          !t &&
            ut() &&
            (this.rtf =
              ((t = e),
              (n = e = void 0 === (e = n) ? {} : e).base,
              (n = _((n = e), be)),
              (n = JSON.stringify([t, n])),
              (r = Me[n]) ||
                ((r = new Intl.RelativeTimeFormat(t, e)), (Me[n] = r)),
              r)));
      }
      var t = e.prototype;
      return (
        (t.format = function (e, t) {
          if (this.rtf) return this.rtf.format(e, t);
          var n = t,
            t = e,
            e = this.opts.numeric,
            r = "long" !== this.opts.style,
            i =
              (void 0 === e && (e = "always"),
              void 0 === r && (r = !1),
              {
                years: ["year", "yr."],
                quarters: ["quarter", "qtr."],
                months: ["month", "mo."],
                weeks: ["week", "wk."],
                days: ["day", "day", "days"],
                hours: ["hour", "hr."],
                minutes: ["minute", "min."],
                seconds: ["second", "sec."],
              }),
            o = -1 === ["hours", "minutes", "seconds"].indexOf(n);
          if ("auto" === e && o) {
            var a = "days" === n;
            switch (t) {
              case 1:
                return a ? "tomorrow" : "next " + i[n][0];
              case -1:
                return a ? "yesterday" : "last " + i[n][0];
              case 0:
                return a ? "today" : "this " + i[n][0];
            }
          }
          var e = Object.is(t, -0) || t < 0,
            t = 1 === (o = Math.abs(t)),
            s = i[n],
            r = r ? (!t && s[2]) || s[1] : t ? i[n][0] : n;
          return e ? o + " " + r + " ago" : "in " + o + " " + r;
        }),
        (t.formatToParts = function (e, t) {
          return this.rtf ? this.rtf.formatToParts(e, t) : [];
        }),
        e
      );
    })(),
    Ze = { firstDay: 1, minimalDays: 4, weekend: [6, 7] },
    b = (() => {
      function o(e, t, n, r, i) {
        var e = ((t) => {
            var n = t.indexOf("-x-");
            if (
              -1 === (n = (t = -1 !== n ? t.substring(0, n) : t).indexOf("-u-"))
            )
              return [t];
            try {
              ((r = Ne(t).resolvedOptions()), (i = t));
            } catch (e) {
              var t = t.substring(0, n),
                r = Ne(t).resolvedOptions(),
                i = t;
            }
            return [i, (n = r).numberingSystem, n.calendar];
          })(e),
          o = e[0],
          a = e[1],
          e = e[2];
        ((this.locale = o),
          (this.numberingSystem = t || a || null),
          (this.outputCalendar = n || e || null),
          (this.weekSettings = r),
          (this.intl =
            ((o = this.locale),
            (t = this.numberingSystem),
            ((a = this.outputCalendar) || t) &&
              (o.includes("-u-") || (o += "-u"), a && (o += "-ca-" + a), t) &&
              (o += "-nu-" + t),
            o)),
          (this.weekdaysCache = { format: {}, standalone: {} }),
          (this.monthsCache = { format: {}, standalone: {} }),
          (this.meridiemCache = null),
          (this.eraCache = {}),
          (this.specifiedLocale = i),
          (this.fastNumbersCached = null));
      }
      ((o.fromOpts = function (e) {
        return o.create(
          e.locale,
          e.numberingSystem,
          e.outputCalendar,
          e.weekSettings,
          e.defaultToEN,
        );
      }),
        (o.create = function (e, t, n, r, i) {
          void 0 === i && (i = !1);
          e = e || O.defaultLocale;
          return new o(
            e ||
              (i
                ? "en-US"
                : (Ie =
                    Ie || new Intl.DateTimeFormat().resolvedOptions().locale)),
            t || O.defaultNumberingSystem,
            n || O.defaultOutputCalendar,
            ft(r) || O.defaultWeekSettings,
            e,
          );
        }),
        (o.resetCache = function () {
          ((Ie = null), (Te = {}), (De = {}), (Me = {}));
        }),
        (o.fromObject = function (e) {
          var e = void 0 === e ? {} : e,
            t = e.locale;
          return o.create(
            t,
            e.numberingSystem,
            e.outputCalendar,
            e.weekSettings,
          );
        }));
      var e = o.prototype;
      return (
        (e.listingMode = function () {
          var e = this.isEnglish(),
            t = !(
              (null !== this.numberingSystem &&
                "latn" !== this.numberingSystem) ||
              (null !== this.outputCalendar &&
                "gregory" !== this.outputCalendar)
            );
          return e && t ? "en" : "intl";
        }),
        (e.clone = function (e) {
          return e && 0 !== Object.getOwnPropertyNames(e).length
            ? o.create(
                e.locale || this.specifiedLocale,
                e.numberingSystem || this.numberingSystem,
                e.outputCalendar || this.outputCalendar,
                ft(e.weekSettings) || this.weekSettings,
                e.defaultToEN || !1,
              )
            : this;
        }),
        (e.redefaultToEN = function (e) {
          return this.clone(
            l({}, (e = void 0 === e ? {} : e), { defaultToEN: !0 }),
          );
        }),
        (e.redefaultToSystem = function (e) {
          return this.clone(
            l({}, (e = void 0 === e ? {} : e), { defaultToEN: !1 }),
          );
        }),
        (e.months = function (n, r) {
          var i = this;
          return (
            void 0 === r && (r = !1),
            Ee(this, n, Et, function () {
              var t = r ? { month: n, day: "numeric" } : { month: n },
                e = r ? "format" : "standalone";
              return (
                i.monthsCache[e][n] ||
                  (i.monthsCache[e][n] = ((e) => {
                    for (var t = [], n = 1; n <= 12; n++) {
                      var r = W.utc(2009, n, 1);
                      t.push(e(r));
                    }
                    return t;
                  })(function (e) {
                    return i.extract(e, t, "month");
                  })),
                i.monthsCache[e][n]
              );
            })
          );
        }),
        (e.weekdays = function (n, r) {
          var i = this;
          return (
            void 0 === r && (r = !1),
            Ee(this, n, Zt, function () {
              var t = r
                  ? {
                      weekday: n,
                      year: "numeric",
                      month: "long",
                      day: "numeric",
                    }
                  : { weekday: n },
                e = r ? "format" : "standalone";
              return (
                i.weekdaysCache[e][n] ||
                  (i.weekdaysCache[e][n] = ((e) => {
                    for (var t = [], n = 1; n <= 7; n++) {
                      var r = W.utc(2016, 11, 13 + n);
                      t.push(e(r));
                    }
                    return t;
                  })(function (e) {
                    return i.extract(e, t, "weekday");
                  })),
                i.weekdaysCache[e][n]
              );
            })
          );
        }),
        (e.meridiems = function () {
          var n = this;
          return Ee(
            this,
            void 0,
            function () {
              return Wt;
            },
            function () {
              var t;
              return (
                n.meridiemCache ||
                  ((t = { hour: "numeric", hourCycle: "h12" }),
                  (n.meridiemCache = [
                    W.utc(2016, 11, 13, 9),
                    W.utc(2016, 11, 13, 19),
                  ].map(function (e) {
                    return n.extract(e, t, "dayperiod");
                  }))),
                n.meridiemCache
              );
            },
          );
        }),
        (e.eras = function (e) {
          var n = this;
          return Ee(this, e, At, function () {
            var t = { era: e };
            return (
              n.eraCache[e] ||
                (n.eraCache[e] = [W.utc(-40, 1, 1), W.utc(2017, 1, 1)].map(
                  function (e) {
                    return n.extract(e, t, "era");
                  },
                )),
              n.eraCache[e]
            );
          });
        }),
        (e.extract = function (e, t, n) {
          e = this.dtFormatter(e, t)
            .formatToParts()
            .find(function (e) {
              return e.type.toLowerCase() === n;
            });
          return e ? e.value : null;
        }),
        (e.numberFormatter = function (e) {
          return new xe(
            this.intl,
            (e = void 0 === e ? {} : e).forceSimple || this.fastNumbers,
            e,
          );
        }),
        (e.dtFormatter = function (e, t) {
          return new Fe(e, this.intl, (t = void 0 === t ? {} : t));
        }),
        (e.relFormatter = function (e) {
          return (
            void 0 === e && (e = {}),
            new Ce(this.intl, this.isEnglish(), e)
          );
        }),
        (e.listFormatter = function (e) {
          return (
            void 0 === e && (e = {}),
            (t = this.intl),
            void 0 === (e = e) && (e = {}),
            (n = JSON.stringify([t, e])),
            (r = Oe[n]) || ((r = new Intl.ListFormat(t, e)), (Oe[n] = r)),
            r
          );
          var t, n, r;
        }),
        (e.isEnglish = function () {
          return (
            "en" === this.locale ||
            "en-us" === this.locale.toLowerCase() ||
            new Intl.DateTimeFormat(this.intl)
              .resolvedOptions()
              .locale.startsWith("en-us")
          );
        }),
        (e.getWeekSettings = function () {
          return (
            this.weekSettings ||
            (lt()
              ? ((e = this.locale),
                (n = Ve[e]) ||
                  ((n =
                    "getWeekInfo" in (t = new Intl.Locale(e))
                      ? t.getWeekInfo()
                      : t.weekInfo),
                  (Ve[e] = n)),
                n)
              : Ze)
          );
          var e, t, n;
        }),
        (e.getStartOfWeek = function () {
          return this.getWeekSettings().firstDay;
        }),
        (e.getMinDaysInFirstWeek = function () {
          return this.getWeekSettings().minimalDays;
        }),
        (e.getWeekendDays = function () {
          return this.getWeekSettings().weekend;
        }),
        (e.equals = function (e) {
          return (
            this.locale === e.locale &&
            this.numberingSystem === e.numberingSystem &&
            this.outputCalendar === e.outputCalendar
          );
        }),
        (e.toString = function () {
          return (
            "Locale(" +
            this.locale +
            ", " +
            this.numberingSystem +
            ", " +
            this.outputCalendar +
            ")"
          );
        }),
        i(o, [
          {
            key: "fastNumbers",
            get: function () {
              var e;
              return (
                null == this.fastNumbersCached &&
                  (this.fastNumbersCached =
                    (!(e = this).numberingSystem ||
                      "latn" === e.numberingSystem) &&
                    ("latn" === e.numberingSystem ||
                      !e.locale ||
                      e.locale.startsWith("en") ||
                      "latn" ===
                        new Intl.DateTimeFormat(e.intl).resolvedOptions()
                          .numberingSystem)),
                this.fastNumbersCached
              );
            },
          },
        ]),
        o
      );
    })(),
    We = null,
    c = ((n) => {
      function t(e) {
        var t = n.call(this) || this;
        return ((t.fixed = e), t);
      }
      (o(t, n),
        (t.instance = function (e) {
          return 0 === e ? t.utcInstance : new t(e);
        }),
        (t.parseSpecifier = function (e) {
          if (e) {
            e = e.match(/^utc(?:([+-]\d{1,2})(?::(\d{2}))?)?$/i);
            if (e) return new t(St(e[1], e[2]));
          }
          return null;
        }));
      var e = t.prototype;
      return (
        (e.offsetName = function () {
          return this.name;
        }),
        (e.formatOffset = function (e, t) {
          return Nt(this.fixed, t);
        }),
        (e.offset = function () {
          return this.fixed;
        }),
        (e.equals = function (e) {
          return "fixed" === e.type && e.fixed === this.fixed;
        }),
        i(
          t,
          [
            {
              key: "type",
              get: function () {
                return "fixed";
              },
            },
            {
              key: "name",
              get: function () {
                return 0 === this.fixed
                  ? "UTC"
                  : "UTC" + Nt(this.fixed, "narrow");
              },
            },
            {
              key: "ianaName",
              get: function () {
                return 0 === this.fixed
                  ? "Etc/UTC"
                  : "Etc/GMT" + Nt(-this.fixed, "narrow");
              },
            },
            {
              key: "isUniversal",
              get: function () {
                return !0;
              },
            },
            {
              key: "isValid",
              get: function () {
                return !0;
              },
            },
          ],
          [
            {
              key: "utcInstance",
              get: function () {
                return (We = null === We ? new t(0) : We);
              },
            },
          ],
        ),
        t
      );
    })(r),
    Le = ((n) => {
      function e(e) {
        var t = n.call(this) || this;
        return ((t.zoneName = e), t);
      }
      o(e, n);
      var t = e.prototype;
      return (
        (t.offsetName = function () {
          return null;
        }),
        (t.formatOffset = function () {
          return "";
        }),
        (t.offset = function () {
          return NaN;
        }),
        (t.equals = function () {
          return !1;
        }),
        i(e, [
          {
            key: "type",
            get: function () {
              return "invalid";
            },
          },
          {
            key: "name",
            get: function () {
              return this.zoneName;
            },
          },
          {
            key: "isUniversal",
            get: function () {
              return !1;
            },
          },
          {
            key: "isValid",
            get: function () {
              return !1;
            },
          },
        ]),
        e
      );
    })(r);
  function S(e, t) {
    var n;
    return null == e
      ? t
      : e instanceof r
        ? e
        : "string" == typeof e
          ? "default" === (n = e.toLowerCase())
            ? t
            : "local" === n || "system" === n
              ? ge.instance
              : "utc" === n || "gmt" === n
                ? c.utcInstance
                : c.parseSpecifier(n) || a.create(e)
          : v(e)
            ? c.instance(e)
            : "object" == typeof e &&
                "offset" in e &&
                "function" == typeof e.offset
              ? e
              : new Le(e);
  }
  var je = {
      arab: "[٠-٩]",
      arabext: "[۰-۹]",
      bali: "[᭐-᭙]",
      beng: "[০-৯]",
      deva: "[०-९]",
      fullwide: "[０-９]",
      gujr: "[૦-૯]",
      hanidec: "[〇|一|二|三|四|五|六|七|八|九]",
      khmr: "[០-៩]",
      knda: "[೦-೯]",
      laoo: "[໐-໙]",
      limb: "[᥆-᥏]",
      mlym: "[൦-൯]",
      mong: "[᠐-᠙]",
      mymr: "[၀-၉]",
      orya: "[୦-୯]",
      tamldec: "[௦-௯]",
      telu: "[౦-౯]",
      thai: "[๐-๙]",
      tibt: "[༠-༩]",
      latn: "\\d",
    },
    ze = {
      arab: [1632, 1641],
      arabext: [1776, 1785],
      bali: [6992, 7001],
      beng: [2534, 2543],
      deva: [2406, 2415],
      fullwide: [65296, 65303],
      gujr: [2790, 2799],
      khmr: [6112, 6121],
      knda: [3302, 3311],
      laoo: [3792, 3801],
      limb: [6470, 6479],
      mlym: [3430, 3439],
      mong: [6160, 6169],
      mymr: [4160, 4169],
      orya: [2918, 2927],
      tamldec: [3046, 3055],
      telu: [3174, 3183],
      thai: [3664, 3673],
      tibt: [3872, 3881],
    },
    Ae = je.hanidec.replace(/[\[|\]]/g, "").split(""),
    s = {};
  function y(e, t) {
    void 0 === t && (t = "");
    e = e.numberingSystem || "latn";
    return (
      s[e] || (s[e] = {}),
      s[e][t] || (s[e][t] = new RegExp("" + je[e] + t)),
      s[e][t]
    );
  }
  var qe,
    _e = function () {
      return Date.now();
    },
    Ue = "system",
    Re = null,
    Pe = null,
    Ye = null,
    He = 60,
    Je = null,
    O = (() => {
      function e() {}
      return (
        (e.resetCaches = function () {
          (b.resetCache(), a.resetCache(), W.resetCache(), (s = {}));
        }),
        i(e, null, [
          {
            key: "now",
            get: function () {
              return _e;
            },
            set: function (e) {
              _e = e;
            },
          },
          {
            key: "defaultZone",
            get: function () {
              return S(Ue, ge.instance);
            },
            set: function (e) {
              Ue = e;
            },
          },
          {
            key: "defaultLocale",
            get: function () {
              return Re;
            },
            set: function (e) {
              Re = e;
            },
          },
          {
            key: "defaultNumberingSystem",
            get: function () {
              return Pe;
            },
            set: function (e) {
              Pe = e;
            },
          },
          {
            key: "defaultOutputCalendar",
            get: function () {
              return Ye;
            },
            set: function (e) {
              Ye = e;
            },
          },
          {
            key: "defaultWeekSettings",
            get: function () {
              return Je;
            },
            set: function (e) {
              Je = ft(e);
            },
          },
          {
            key: "twoDigitCutoffYear",
            get: function () {
              return He;
            },
            set: function (e) {
              He = e % 100;
            },
          },
          {
            key: "throwOnInvalid",
            get: function () {
              return qe;
            },
            set: function (e) {
              qe = e;
            },
          },
        ]),
        e
      );
    })(),
    f = (() => {
      function e(e, t) {
        ((this.reason = e), (this.explanation = t));
      }
      return (
        (e.prototype.toMessage = function () {
          return this.explanation
            ? this.reason + ": " + this.explanation
            : this.reason;
        }),
        e
      );
    })(),
    Ge = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
    $e = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
  function T(e, t) {
    return new f(
      "unit out of range",
      "you specified " +
        t +
        " (of type " +
        typeof t +
        ") as a " +
        e +
        ", which is invalid",
    );
  }
  function Be(e, t, n) {
    ((t = new Date(Date.UTC(e, t - 1, n))),
      e < 100 && 0 <= e && t.setUTCFullYear(t.getUTCFullYear() - 1900),
      (n = t.getUTCDay()));
    return 0 === n ? 7 : n;
  }
  function Qe(e, t, n) {
    return n + (mt(e) ? $e : Ge)[t - 1];
  }
  function Ke(e, t) {
    var e = mt(e) ? $e : Ge,
      n = e.findIndex(function (e) {
        return e < t;
      });
    return { month: n + 1, day: t - e[n] };
  }
  function Xe(e, t) {
    return ((e - t + 7) % 7) + 1;
  }
  function et(e, t, n) {
    (void 0 === t && (t = 4), void 0 === n && (n = 1));
    var r,
      i = e.year,
      o = e.month,
      a = e.day,
      s = Qe(i, o, a),
      o = Xe(Be(i, o, a), n),
      a = Math.floor((s - o + 14 - t) / 7);
    return (
      a < 1
        ? (a = kt((r = i - 1), t, n))
        : a > kt(i, t, n)
          ? ((r = i + 1), (a = 1))
          : (r = i),
      l({ weekYear: r, weekNumber: a, weekday: o }, Dt(e))
    );
  }
  function tt(e, t, n) {
    void 0 === n && (n = 1);
    var r,
      i = e.weekYear,
      o = e.weekNumber,
      a = e.weekday,
      n = Xe(Be(i, 1, (t = void 0 === t ? 4 : t)), n),
      s = yt(i),
      o = 7 * o + a - n - 7 + t,
      a =
        (o < 1
          ? (o += yt((r = i - 1)))
          : s < o
            ? ((r = i + 1), (o -= yt(i)))
            : (r = i),
        Ke(r, o));
    return l({ year: r, month: a.month, day: a.day }, Dt(e));
  }
  function nt(e) {
    var t = e.year;
    return l({ year: t, ordinal: Qe(t, e.month, e.day) }, Dt(e));
  }
  function rt(e) {
    var t = e.year,
      n = Ke(t, e.ordinal);
    return l({ year: t, month: n.month, day: n.day }, Dt(e));
  }
  function it(e, t) {
    if (N(e.localWeekday) && N(e.localWeekNumber) && N(e.localWeekYear))
      return { minDaysInFirstWeek: 4, startOfWeek: 1 };
    if (N(e.weekday) && N(e.weekNumber) && N(e.weekYear))
      return (
        N(e.localWeekday) || (e.weekday = e.localWeekday),
        N(e.localWeekNumber) || (e.weekNumber = e.localWeekNumber),
        N(e.localWeekYear) || (e.weekYear = e.localWeekYear),
        delete e.localWeekday,
        delete e.localWeekNumber,
        delete e.localWeekYear,
        {
          minDaysInFirstWeek: t.getMinDaysInFirstWeek(),
          startOfWeek: t.getStartOfWeek(),
        }
      );
    throw new w(
      "Cannot mix locale-based week fields with ISO-based week fields",
    );
  }
  function ot(e) {
    var t = st(e.year),
      n = D(e.month, 1, 12),
      r = D(e.day, 1, vt(e.year, e.month));
    return t
      ? n
        ? !r && T("day", e.day)
        : T("month", e.month)
      : T("year", e.year);
  }
  function at(e) {
    var t = e.hour,
      n = e.minute,
      r = e.second,
      e = e.millisecond,
      i = D(t, 0, 23) || (24 === t && 0 === n && 0 === r && 0 === e),
      o = D(n, 0, 59),
      a = D(r, 0, 59),
      s = D(e, 0, 999);
    return i
      ? o
        ? a
          ? !s && T("millisecond", e)
          : T("second", r)
        : T("minute", n)
      : T("hour", t);
  }
  function N(e) {
    return void 0 === e;
  }
  function v(e) {
    return "number" == typeof e;
  }
  function st(e) {
    return "number" == typeof e && e % 1 == 0;
  }
  function ut() {
    try {
      return "undefined" != typeof Intl && !!Intl.RelativeTimeFormat;
    } catch (e) {
      return !1;
    }
  }
  function lt() {
    try {
      return (
        "undefined" != typeof Intl &&
        !!Intl.Locale &&
        ("weekInfo" in Intl.Locale.prototype ||
          "getWeekInfo" in Intl.Locale.prototype)
      );
    } catch (e) {
      return !1;
    }
  }
  function ct(e, n, r) {
    if (0 !== e.length)
      return e.reduce(function (e, t) {
        t = [n(t), t];
        return e && r(e[0], t[0]) === e[0] ? e : t;
      }, null)[1];
  }
  function d(e, t) {
    return Object.prototype.hasOwnProperty.call(e, t);
  }
  function ft(e) {
    if (null == e) return null;
    if ("object" != typeof e) throw new u("Week settings must be an object");
    if (
      D(e.firstDay, 1, 7) &&
      D(e.minimalDays, 1, 7) &&
      Array.isArray(e.weekend) &&
      !e.weekend.some(function (e) {
        return !D(e, 1, 7);
      })
    )
      return {
        firstDay: e.firstDay,
        minimalDays: e.minimalDays,
        weekend: Array.from(e.weekend),
      };
    throw new u("Invalid week settings");
  }
  function D(e, t, n) {
    return st(e) && t <= e && e <= n;
  }
  function h(e, t) {
    void 0 === t && (t = 2);
    e = e < 0 ? "-" + ("" + -e).padStart(t, "0") : ("" + e).padStart(t, "0");
    return e;
  }
  function m(e) {
    if (null != e && "" !== e) return parseInt(e, 10);
  }
  function g(e) {
    if (null != e && "" !== e) return parseFloat(e);
  }
  function dt(e) {
    if (null != e && "" !== e)
      return ((e = 1e3 * parseFloat("0." + e)), Math.floor(e));
  }
  function ht(e, t, n) {
    void 0 === n && (n = !1);
    t = Math.pow(10, t);
    return (n ? Math.trunc : Math.round)(e * t) / t;
  }
  function mt(e) {
    return e % 4 == 0 && (e % 100 != 0 || e % 400 == 0);
  }
  function yt(e) {
    return mt(e) ? 366 : 365;
  }
  function vt(e, t) {
    var n,
      r = (r = t - 1) - (n = 12) * Math.floor(r / n) + 1;
    return 2 == r
      ? mt(e + (t - r) / 12)
        ? 29
        : 28
      : [31, null, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][r - 1];
  }
  function gt(e) {
    var t = Date.UTC(
      e.year,
      e.month - 1,
      e.day,
      e.hour,
      e.minute,
      e.second,
      e.millisecond,
    );
    return (
      e.year < 100 &&
        0 <= e.year &&
        (t = new Date(t)).setUTCFullYear(e.year, e.month - 1, e.day),
      +t
    );
  }
  function pt(e, t, n) {
    return -Xe(Be(e, 1, t), n) + t - 1;
  }
  function kt(e, t, n) {
    var r = pt(e, (t = void 0 === t ? 4 : t), (n = void 0 === n ? 1 : n)),
      t = pt(e + 1, t, n);
    return (yt(e) - r + t) / 7;
  }
  function wt(e) {
    return 99 < e ? e : e > O.twoDigitCutoffYear ? 1900 + e : 2e3 + e;
  }
  function bt(e, t, n, r) {
    void 0 === r && (r = null);
    var e = new Date(e),
      i = {
        hourCycle: "h23",
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
      },
      r = (r && (i.timeZone = r), l({ timeZoneName: t }, i)),
      t = new Intl.DateTimeFormat(n, r).formatToParts(e).find(function (e) {
        return "timezonename" === e.type.toLowerCase();
      });
    return t ? t.value : null;
  }
  function St(e, t) {
    ((e = parseInt(e, 10)),
      Number.isNaN(e) && (e = 0),
      (t = parseInt(t, 10) || 0));
    return 60 * e + (e < 0 || Object.is(e, -0) ? -t : t);
  }
  function Ot(e) {
    var t = Number(e);
    if ("boolean" == typeof e || "" === e || Number.isNaN(t))
      throw new u("Invalid unit value " + e);
    return t;
  }
  function Tt(e, t) {
    var n,
      r,
      i = {};
    for (n in e) d(e, n) && null != (r = e[n]) && (i[t(n)] = Ot(r));
    return i;
  }
  function Nt(e, t) {
    var n = Math.trunc(Math.abs(e / 60)),
      r = Math.trunc(Math.abs(e % 60)),
      i = 0 <= e ? "+" : "-";
    switch (t) {
      case "short":
        return i + h(n, 2) + ":" + h(r, 2);
      case "narrow":
        return i + n + (0 < r ? ":" + r : "");
      case "techie":
        return i + h(n, 2) + h(r, 2);
      default:
        throw new RangeError(
          "Value format " + t + " is out of range for property format",
        );
    }
  }
  function Dt(e) {
    return (
      (n = e),
      ["hour", "minute", "second", "millisecond"].reduce(function (e, t) {
        return ((e[t] = n[t]), e);
      }, {})
    );
    var n;
  }
  var Mt = [
      "January",
      "February",
      "March",
      "April",
      "May",
      "June",
      "July",
      "August",
      "September",
      "October",
      "November",
      "December",
    ],
    It = [
      "Jan",
      "Feb",
      "Mar",
      "Apr",
      "May",
      "Jun",
      "Jul",
      "Aug",
      "Sep",
      "Oct",
      "Nov",
      "Dec",
    ],
    Vt = ["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"];
  function Et(e) {
    switch (e) {
      case "narrow":
        return [].concat(Vt);
      case "short":
        return [].concat(It);
      case "long":
        return [].concat(Mt);
      case "numeric":
        return ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12"];
      case "2-digit":
        return [
          "01",
          "02",
          "03",
          "04",
          "05",
          "06",
          "07",
          "08",
          "09",
          "10",
          "11",
          "12",
        ];
      default:
        return null;
    }
  }
  var xt = [
      "Monday",
      "Tuesday",
      "Wednesday",
      "Thursday",
      "Friday",
      "Saturday",
      "Sunday",
    ],
    Ft = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
    Ct = ["M", "T", "W", "T", "F", "S", "S"];
  function Zt(e) {
    switch (e) {
      case "narrow":
        return [].concat(Ct);
      case "short":
        return [].concat(Ft);
      case "long":
        return [].concat(xt);
      case "numeric":
        return ["1", "2", "3", "4", "5", "6", "7"];
      default:
        return null;
    }
  }
  var Wt = ["AM", "PM"],
    Lt = ["Before Christ", "Anno Domini"],
    jt = ["BC", "AD"],
    zt = ["B", "A"];
  function At(e) {
    switch (e) {
      case "narrow":
        return [].concat(zt);
      case "short":
        return [].concat(jt);
      case "long":
        return [].concat(Lt);
      default:
        return null;
    }
  }
  function qt(e, t) {
    for (var n = "", r = R(e); !(i = r()).done; ) {
      var i = i.value;
      i.literal ? (n += i.val) : (n += t(i.val));
    }
    return n;
  }
  var _t = {
      D: G,
      DD: $,
      DDD: Q,
      DDDD: K,
      t: X,
      tt: ee,
      ttt: te,
      tttt: ne,
      T: re,
      TT: ie,
      TTT: oe,
      TTTT: ae,
      f: se,
      ff: le,
      fff: de,
      ffff: me,
      F: ue,
      FF: ce,
      FFF: he,
      FFFF: ye,
    },
    p = (() => {
      function d(e, t) {
        ((this.opts = t), (this.loc = e), (this.systemLoc = null));
      }
      ((d.create = function (e, t) {
        return new d(e, (t = void 0 === t ? {} : t));
      }),
        (d.parseFormat = function (e) {
          for (var t = null, n = "", r = !1, i = [], o = 0; o < e.length; o++) {
            var a = e.charAt(o);
            "'" === a
              ? (0 < n.length &&
                  i.push({ literal: r || /^\s+$/.test(n), val: n }),
                (t = null),
                (n = ""),
                (r = !r))
              : r || a === t
                ? (n += a)
                : (0 < n.length && i.push({ literal: /^\s+$/.test(n), val: n }),
                  (t = n = a));
          }
          return (
            0 < n.length && i.push({ literal: r || /^\s+$/.test(n), val: n }),
            i
          );
        }),
        (d.macroTokenToFormatOpts = function (e) {
          return _t[e];
        }));
      var e = d.prototype;
      return (
        (e.formatWithSystemDefault = function (e, t) {
          return (
            null === this.systemLoc &&
              (this.systemLoc = this.loc.redefaultToSystem()),
            this.systemLoc.dtFormatter(e, l({}, this.opts, t)).format()
          );
        }),
        (e.dtFormatter = function (e, t) {
          return this.loc.dtFormatter(
            e,
            l({}, this.opts, (t = void 0 === t ? {} : t)),
          );
        }),
        (e.formatDateTime = function (e, t) {
          return this.dtFormatter(e, t).format();
        }),
        (e.formatDateTimeParts = function (e, t) {
          return this.dtFormatter(e, t).formatToParts();
        }),
        (e.formatInterval = function (e, t) {
          return this.dtFormatter(e.start, t).dtf.formatRange(
            e.start.toJSDate(),
            e.end.toJSDate(),
          );
        }),
        (e.resolvedOptions = function (e, t) {
          return this.dtFormatter(e, t).resolvedOptions();
        }),
        (e.num = function (e, t) {
          var n;
          return (
            void 0 === t && (t = 0),
            this.opts.forceSimple
              ? h(e, t)
              : ((n = l({}, this.opts)),
                0 < t && (n.padTo = t),
                this.loc.numberFormatter(n).format(e))
          );
        }),
        (e.formatDateTimeFromString = function (r, e) {
          var n = this,
            i = "en" === this.loc.listingMode(),
            t =
              this.loc.outputCalendar && "gregory" !== this.loc.outputCalendar,
            o = function (e, t) {
              return n.loc.extract(r, e, t);
            },
            a = function (e) {
              return r.isOffsetFixed && 0 === r.offset && e.allowZ
                ? "Z"
                : r.isValid
                  ? r.zone.formatOffset(r.ts, e.format)
                  : "";
            },
            s = function () {
              return i
                ? Wt[r.hour < 12 ? 0 : 1]
                : o({ hour: "numeric", hourCycle: "h12" }, "dayperiod");
            },
            u = function (e, t) {
              return i
                ? ((n = r), Et(e)[n.month - 1])
                : o(t ? { month: e } : { month: e, day: "numeric" }, "month");
              var n;
            },
            l = function (e, t) {
              return i
                ? ((n = r), Zt(e)[n.weekday - 1])
                : o(
                    t
                      ? { weekday: e }
                      : { weekday: e, month: "long", day: "numeric" },
                    "weekday",
                  );
              var n;
            },
            c = function (e) {
              var t = d.macroTokenToFormatOpts(e);
              return t ? n.formatWithSystemDefault(r, t) : e;
            },
            f = function (e) {
              return i
                ? ((t = r), At(e)[t.year < 0 ? 0 : 1])
                : o({ era: e }, "era");
              var t;
            };
          return qt(d.parseFormat(e), function (e) {
            switch (e) {
              case "S":
                return n.num(r.millisecond);
              case "u":
              case "SSS":
                return n.num(r.millisecond, 3);
              case "s":
                return n.num(r.second);
              case "ss":
                return n.num(r.second, 2);
              case "uu":
                return n.num(Math.floor(r.millisecond / 10), 2);
              case "uuu":
                return n.num(Math.floor(r.millisecond / 100));
              case "m":
                return n.num(r.minute);
              case "mm":
                return n.num(r.minute, 2);
              case "h":
                return n.num(r.hour % 12 == 0 ? 12 : r.hour % 12);
              case "hh":
                return n.num(r.hour % 12 == 0 ? 12 : r.hour % 12, 2);
              case "H":
                return n.num(r.hour);
              case "HH":
                return n.num(r.hour, 2);
              case "Z":
                return a({ format: "narrow", allowZ: n.opts.allowZ });
              case "ZZ":
                return a({ format: "short", allowZ: n.opts.allowZ });
              case "ZZZ":
                return a({ format: "techie", allowZ: n.opts.allowZ });
              case "ZZZZ":
                return r.zone.offsetName(r.ts, {
                  format: "short",
                  locale: n.loc.locale,
                });
              case "ZZZZZ":
                return r.zone.offsetName(r.ts, {
                  format: "long",
                  locale: n.loc.locale,
                });
              case "z":
                return r.zoneName;
              case "a":
                return s();
              case "d":
                return t ? o({ day: "numeric" }, "day") : n.num(r.day);
              case "dd":
                return t ? o({ day: "2-digit" }, "day") : n.num(r.day, 2);
              case "c":
                return n.num(r.weekday);
              case "ccc":
                return l("short", !0);
              case "cccc":
                return l("long", !0);
              case "ccccc":
                return l("narrow", !0);
              case "E":
                return n.num(r.weekday);
              case "EEE":
                return l("short", !1);
              case "EEEE":
                return l("long", !1);
              case "EEEEE":
                return l("narrow", !1);
              case "L":
                return t
                  ? o({ month: "numeric", day: "numeric" }, "month")
                  : n.num(r.month);
              case "LL":
                return t
                  ? o({ month: "2-digit", day: "numeric" }, "month")
                  : n.num(r.month, 2);
              case "LLL":
                return u("short", !0);
              case "LLLL":
                return u("long", !0);
              case "LLLLL":
                return u("narrow", !0);
              case "M":
                return t ? o({ month: "numeric" }, "month") : n.num(r.month);
              case "MM":
                return t ? o({ month: "2-digit" }, "month") : n.num(r.month, 2);
              case "MMM":
                return u("short", !1);
              case "MMMM":
                return u("long", !1);
              case "MMMMM":
                return u("narrow", !1);
              case "y":
                return t ? o({ year: "numeric" }, "year") : n.num(r.year);
              case "yy":
                return t
                  ? o({ year: "2-digit" }, "year")
                  : n.num(r.year.toString().slice(-2), 2);
              case "yyyy":
                return t ? o({ year: "numeric" }, "year") : n.num(r.year, 4);
              case "yyyyyy":
                return t ? o({ year: "numeric" }, "year") : n.num(r.year, 6);
              case "G":
                return f("short");
              case "GG":
                return f("long");
              case "GGGGG":
                return f("narrow");
              case "kk":
                return n.num(r.weekYear.toString().slice(-2), 2);
              case "kkkk":
                return n.num(r.weekYear, 4);
              case "W":
                return n.num(r.weekNumber);
              case "WW":
                return n.num(r.weekNumber, 2);
              case "n":
                return n.num(r.localWeekNumber);
              case "nn":
                return n.num(r.localWeekNumber, 2);
              case "ii":
                return n.num(r.localWeekYear.toString().slice(-2), 2);
              case "iiii":
                return n.num(r.localWeekYear, 4);
              case "o":
                return n.num(r.ordinal);
              case "ooo":
                return n.num(r.ordinal, 3);
              case "q":
                return n.num(r.quarter);
              case "qq":
                return n.num(r.quarter, 2);
              case "X":
                return n.num(Math.floor(r.ts / 1e3));
              case "x":
                return n.num(r.ts);
              default:
                return c(e);
            }
          });
        }),
        (e.formatDurationFromString = function (e, t) {
          var n,
            r = this,
            i = function (e) {
              switch (e[0]) {
                case "S":
                  return "millisecond";
                case "s":
                  return "second";
                case "m":
                  return "minute";
                case "h":
                  return "hour";
                case "d":
                  return "day";
                case "w":
                  return "week";
                case "M":
                  return "month";
                case "y":
                  return "year";
                default:
                  return null;
              }
            },
            t = d.parseFormat(t),
            o = t.reduce(function (e, t) {
              return t.literal ? e : e.concat(t.val);
            }, []),
            e = e.shiftTo.apply(
              e,
              o.map(i).filter(function (e) {
                return e;
              }),
            );
          return qt(
            t,
            ((n = e),
            function (e) {
              var t = i(e);
              return t ? r.num(n.get(t), e.length) : e;
            }),
          );
        }),
        d
      );
    })(),
    t =
      /[A-Za-z_+-]{1,256}(?::?\/[A-Za-z0-9_+-]{1,256}(?:\/[A-Za-z0-9_+-]{1,256})?)?/;
  function Ut() {
    for (var e = arguments.length, t = new Array(e), n = 0; n < e; n++)
      t[n] = arguments[n];
    var r = t.reduce(function (e, t) {
      return e + t.source;
    }, "");
    return RegExp("^" + r + "$");
  }
  function Rt() {
    for (var e = arguments.length, t = new Array(e), n = 0; n < e; n++)
      t[n] = arguments[n];
    return function (o) {
      return t
        .reduce(
          function (e, t) {
            var n = e[0],
              r = e[1],
              t = t(o, e[2]),
              e = t[0],
              i = t[1],
              t = t[2];
            return [l({}, n, e), i || r, t];
          },
          [{}, null, 1],
        )
        .slice(0, 2);
    };
  }
  function Pt(e) {
    if (null != e) {
      for (
        var t = arguments.length, n = new Array(1 < t ? t - 1 : 0), r = 1;
        r < t;
        r++
      )
        n[r - 1] = arguments[r];
      for (var i = 0, o = n; i < o.length; i++) {
        var a = o[i],
          s = a[0],
          a = a[1],
          s = s.exec(e);
        if (s) return a(s);
      }
    }
    return [null, null];
  }
  function Yt() {
    for (var e = arguments.length, i = new Array(e), t = 0; t < e; t++)
      i[t] = arguments[t];
    return function (e, t) {
      for (var n = {}, r = 0; r < i.length; r++) n[i[r]] = m(e[t + r]);
      return [n, null, t + r];
    };
  }
  var k = /(?:(Z)|([+-]\d\d)(?::?(\d\d))?)/,
    M = /(\d\d)(?::?(\d\d)(?::?(\d\d)(?:[.,](\d{1,30}))?)?)?/,
    Ht = RegExp(
      M.source + ("(?:" + k.source + "?(?:\\[(" + t.source + ")\\])?)?"),
    ),
    I = RegExp("(?:T" + Ht.source + ")?"),
    Jt = Yt("weekYear", "weekNumber", "weekDay"),
    Gt = Yt("year", "ordinal"),
    k = RegExp(M.source + " ?(?:" + k.source + "|(" + t.source + "))?"),
    t = RegExp("(?: " + k.source + ")?");
  function $t(e, t, n) {
    e = e[t];
    return N(e) ? n : m(e);
  }
  function Bt(e, t) {
    return [
      {
        hours: $t(e, t, 0),
        minutes: $t(e, t + 1, 0),
        seconds: $t(e, t + 2, 0),
        milliseconds: dt(e[t + 3]),
      },
      null,
      t + 4,
    ];
  }
  function Qt(e, t) {
    var n = !e[t] && !e[t + 1],
      e = St(e[t + 1], e[t + 2]);
    return [{}, n ? null : c.instance(e), t + 3];
  }
  function Kt(e, t) {
    return [{}, e[t] ? a.create(e[t]) : null, t + 1];
  }
  var Xt = RegExp("^T?" + M.source + "$"),
    en =
      /^-?P(?:(?:(-?\d{1,20}(?:\.\d{1,20})?)Y)?(?:(-?\d{1,20}(?:\.\d{1,20})?)M)?(?:(-?\d{1,20}(?:\.\d{1,20})?)W)?(?:(-?\d{1,20}(?:\.\d{1,20})?)D)?(?:T(?:(-?\d{1,20}(?:\.\d{1,20})?)H)?(?:(-?\d{1,20}(?:\.\d{1,20})?)M)?(?:(-?\d{1,20})(?:[.,](-?\d{1,20}))?S)?)?)$/;
  function tn(e) {
    function t(e, t) {
      return (
        void 0 === t && (t = !1),
        void 0 !== e && (t || (e && c)) ? -e : e
      );
    }
    var n = e[0],
      r = e[2],
      i = e[3],
      o = e[4],
      a = e[5],
      s = e[6],
      u = e[7],
      l = e[8],
      c = "-" === n[0],
      n = u && "-" === u[0];
    return [
      {
        years: t(g(e[1])),
        months: t(g(r)),
        weeks: t(g(i)),
        days: t(g(o)),
        hours: t(g(a)),
        minutes: t(g(s)),
        seconds: t(g(u), "-0" === u),
        milliseconds: t(dt(l), n),
      },
    ];
  }
  var nn = {
    GMT: 0,
    EDT: -240,
    EST: -300,
    CDT: -300,
    CST: -360,
    MDT: -360,
    MST: -420,
    PDT: -420,
    PST: -480,
  };
  function rn(e, t, n, r, i, o, a) {
    t = {
      year: 2 === t.length ? wt(m(t)) : m(t),
      month: It.indexOf(n) + 1,
      day: m(r),
      hour: m(i),
      minute: m(o),
    };
    return (
      a && (t.second = m(a)),
      e && (t.weekday = 3 < e.length ? xt.indexOf(e) + 1 : Ft.indexOf(e) + 1),
      t
    );
  }
  var on =
    /^(?:(Mon|Tue|Wed|Thu|Fri|Sat|Sun),\s)?(\d{1,2})\s(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)\s(\d{2,4})\s(\d\d):(\d\d)(?::(\d\d))?\s(?:(UT|GMT|[ECMP][SD]T)|([Zz])|(?:([+-]\d\d)(\d\d)))$/;
  function an(e) {
    var t = e[1],
      n = e[8],
      r = e[9],
      i = e[10],
      o = e[11],
      t = rn(t, e[4], e[3], e[2], e[5], e[6], e[7]),
      e = n ? nn[n] : r ? 0 : St(i, o);
    return [t, new c(e)];
  }
  var sn =
      /^(Mon|Tue|Wed|Thu|Fri|Sat|Sun), (\d\d) (Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) (\d{4}) (\d\d):(\d\d):(\d\d) GMT$/,
    un =
      /^(Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday), (\d\d)-(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)-(\d\d) (\d\d):(\d\d):(\d\d) GMT$/,
    ln =
      /^(Mon|Tue|Wed|Thu|Fri|Sat|Sun) (Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) ( \d|\d\d) (\d\d):(\d\d):(\d\d) (\d{4})$/;
  function cn(e) {
    return [rn(e[1], e[4], e[3], e[2], e[5], e[6], e[7]), c.utcInstance];
  }
  function fn(e) {
    return [rn(e[1], e[7], e[2], e[3], e[4], e[5], e[6]), c.utcInstance];
  }
  var dn = Ut(/([+-]\d{6}|\d{4})(?:-?(\d\d)(?:-?(\d\d))?)?/, I),
    hn = Ut(/(\d{4})-?W(\d\d)(?:-?(\d))?/, I),
    mn = Ut(/(\d{4})-?(\d{3})/, I),
    yn = Ut(Ht),
    vn = Rt(
      function (e, t) {
        return [
          { year: $t(e, t), month: $t(e, t + 1, 1), day: $t(e, t + 2, 1) },
          null,
          t + 3,
        ];
      },
      Bt,
      Qt,
      Kt,
    ),
    gn = Rt(Jt, Bt, Qt, Kt),
    pn = Rt(Gt, Bt, Qt, Kt),
    kn = Rt(Bt, Qt, Kt),
    wn = Rt(Bt),
    bn = Ut(/(\d{4})-(\d\d)-(\d\d)/, t),
    Sn = Ut(k),
    On = Rt(Bt, Qt, Kt),
    Tn = "Invalid Duration",
    Nn = l(
      {
        years: {
          quarters: 4,
          months: 12,
          weeks: 52,
          days: 365,
          hours: 8760,
          minutes: 525600,
          seconds: 31536e3,
          milliseconds: 31536e6,
        },
        quarters: {
          months: 3,
          weeks: 13,
          days: 91,
          hours: 2184,
          minutes: 131040,
          seconds: 7862400,
          milliseconds: 78624e5,
        },
        months: {
          weeks: 4,
          days: 30,
          hours: 720,
          minutes: 43200,
          seconds: 2592e3,
          milliseconds: 2592e6,
        },
      },
      (M = {
        weeks: {
          days: 7,
          hours: 168,
          minutes: 10080,
          seconds: 604800,
          milliseconds: 6048e5,
        },
        days: { hours: 24, minutes: 1440, seconds: 86400, milliseconds: 864e5 },
        hours: { minutes: 60, seconds: 3600, milliseconds: 36e5 },
        minutes: { seconds: 60, milliseconds: 6e4 },
        seconds: { milliseconds: 1e3 },
      }),
    ),
    Dn = l(
      {
        years: {
          quarters: 4,
          months: 12,
          weeks: (I = 365.2425) / 7,
          days: I,
          hours: 24 * I,
          minutes: 525949.2,
          seconds: 525949.2 * 60,
          milliseconds: 525949.2 * 60 * 1e3,
        },
        quarters: {
          months: 3,
          weeks: I / 28,
          days: I / 4,
          hours: (24 * I) / 4,
          minutes: 131487.3,
          seconds: (525949.2 * 60) / 4,
          milliseconds: 7889237999.999999,
        },
        months: {
          weeks: (Ht = 30.436875) / 7,
          days: Ht,
          hours: 24 * Ht,
          minutes: 43829.1,
          seconds: 2629746,
          milliseconds: 2629746e3,
        },
      },
      M,
    ),
    V = [
      "years",
      "quarters",
      "months",
      "weeks",
      "days",
      "hours",
      "minutes",
      "seconds",
      "milliseconds",
    ],
    Mn = V.slice(0).reverse();
  function E(e, t, n) {
    n = {
      values: (n = void 0 === n ? !1 : n)
        ? t.values
        : l({}, e.values, t.values || {}),
      loc: e.loc.clone(t.loc),
      conversionAccuracy: t.conversionAccuracy || e.conversionAccuracy,
      matrix: t.matrix || e.matrix,
    };
    return new x(n);
  }
  function In(e, t) {
    for (
      var n, r = null != (n = t.milliseconds) ? n : 0, i = R(Mn.slice(1));
      !(o = i()).done;

    ) {
      var o = o.value;
      t[o] && (r += t[o] * e[o].milliseconds);
    }
    return r;
  }
  function Vn(i, o) {
    var a = In(i, o) < 0 ? -1 : 1;
    (V.reduceRight(function (e, t) {
      var n, r;
      return N(o[t])
        ? e
        : (e &&
            ((r = o[e] * a),
            (n = i[t][e]),
            (r = Math.floor(r / n)),
            (o[t] += r * a),
            (o[e] -= r * n * a)),
          t);
    }, null),
      V.reduce(function (e, t) {
        var n;
        return N(o[t])
          ? e
          : (e && ((n = o[e] % 1), (o[e] -= n), (o[t] += n * i[e][t])), t);
      }, null));
  }
  var x = ((e) => {
      function m(e) {
        var t = "longterm" === e.conversionAccuracy || !1,
          n = t ? Dn : Nn;
        (e.matrix && (n = e.matrix),
          (this.values = e.values),
          (this.loc = e.loc || b.create()),
          (this.conversionAccuracy = t ? "longterm" : "casual"),
          (this.invalid = e.invalid || null),
          (this.matrix = n),
          (this.isLuxonDuration = !0));
      }
      ((m.fromMillis = function (e, t) {
        return m.fromObject({ milliseconds: e }, t);
      }),
        (m.fromObject = function (e, t) {
          if ((void 0 === t && (t = {}), null == e || "object" != typeof e))
            throw new u(
              "Duration.fromObject: argument expected to be an object, got " +
                (null === e ? "null" : typeof e),
            );
          return new m({
            values: Tt(e, m.normalizeUnit),
            loc: b.fromObject(t),
            conversionAccuracy: t.conversionAccuracy,
            matrix: t.matrix,
          });
        }),
        (m.fromDurationLike = function (e) {
          if (v(e)) return m.fromMillis(e);
          if (m.isDuration(e)) return e;
          if ("object" == typeof e) return m.fromObject(e);
          throw new u(
            "Unknown duration argument " + e + " of type " + typeof e,
          );
        }),
        (m.fromISO = function (e, t) {
          var n = Pt(e, [en, tn])[0];
          return n
            ? m.fromObject(n, t)
            : m.invalid(
                "unparsable",
                'the input "' + e + "\" can't be parsed as ISO 8601",
              );
        }),
        (m.fromISOTime = function (e, t) {
          var n = Pt(e, [Xt, wn])[0];
          return n
            ? m.fromObject(n, t)
            : m.invalid(
                "unparsable",
                'the input "' + e + "\" can't be parsed as ISO 8601",
              );
        }),
        (m.invalid = function (e, t) {
          if ((void 0 === t && (t = null), !e))
            throw new u("need to specify a reason the Duration is invalid");
          e = e instanceof f ? e : new f(e, t);
          if (O.throwOnInvalid) throw new H(e);
          return new m({ invalid: e });
        }),
        (m.normalizeUnit = function (e) {
          var t = {
            year: "years",
            years: "years",
            quarter: "quarters",
            quarters: "quarters",
            month: "months",
            months: "months",
            week: "weeks",
            weeks: "weeks",
            day: "days",
            days: "days",
            hour: "hours",
            hours: "hours",
            minute: "minutes",
            minutes: "minutes",
            second: "seconds",
            seconds: "seconds",
            millisecond: "milliseconds",
            milliseconds: "milliseconds",
          }[e && e.toLowerCase()];
          if (t) return t;
          throw new J(e);
        }),
        (m.isDuration = function (e) {
          return (e && e.isLuxonDuration) || !1;
        }));
      var t = m.prototype;
      return (
        (t.toFormat = function (e, t) {
          t = l({}, (t = void 0 === t ? {} : t), {
            floor: !1 !== t.round && !1 !== t.floor,
          });
          return this.isValid
            ? p.create(this.loc, t).formatDurationFromString(this, e)
            : Tn;
        }),
        (t.toHuman = function (n) {
          var e,
            r = this;
          return (
            void 0 === n && (n = {}),
            this.isValid
              ? ((e = V.map(function (e) {
                  var t = r.values[e];
                  return N(t)
                    ? null
                    : r.loc
                        .numberFormatter(
                          l({ style: "unit", unitDisplay: "long" }, n, {
                            unit: e.slice(0, -1),
                          }),
                        )
                        .format(t);
                }).filter(function (e) {
                  return e;
                })),
                this.loc
                  .listFormatter(
                    l(
                      { type: "conjunction", style: n.listStyle || "narrow" },
                      n,
                    ),
                  )
                  .format(e))
              : Tn
          );
        }),
        (t.toObject = function () {
          return this.isValid ? l({}, this.values) : {};
        }),
        (t.toISO = function () {
          var e;
          return this.isValid
            ? ((e = "P"),
              0 !== this.years && (e += this.years + "Y"),
              (0 === this.months && 0 === this.quarters) ||
                (e += this.months + 3 * this.quarters + "M"),
              0 !== this.weeks && (e += this.weeks + "W"),
              0 !== this.days && (e += this.days + "D"),
              (0 === this.hours &&
                0 === this.minutes &&
                0 === this.seconds &&
                0 === this.milliseconds) ||
                (e += "T"),
              0 !== this.hours && (e += this.hours + "H"),
              0 !== this.minutes && (e += this.minutes + "M"),
              (0 === this.seconds && 0 === this.milliseconds) ||
                (e += ht(this.seconds + this.milliseconds / 1e3, 3) + "S"),
              "P" === e && (e += "T0S"),
              e)
            : null;
        }),
        (t.toISOTime = function (e) {
          var t;
          return (
            void 0 === e && (e = {}),
            !this.isValid || (t = this.toMillis()) < 0 || 864e5 <= t
              ? null
              : ((e = l(
                  {
                    suppressMilliseconds: !1,
                    suppressSeconds: !1,
                    includePrefix: !1,
                    format: "extended",
                  },
                  e,
                  { includeOffset: !1 },
                )),
                W.fromMillis(t, { zone: "UTC" }).toISOTime(e))
          );
        }),
        (t.toJSON = function () {
          return this.toISO();
        }),
        (t.toString = function () {
          return this.toISO();
        }),
        (t[e] = function () {
          return this.isValid
            ? "Duration { values: " + JSON.stringify(this.values) + " }"
            : "Duration { Invalid, reason: " + this.invalidReason + " }";
        }),
        (t.toMillis = function () {
          return this.isValid ? In(this.matrix, this.values) : NaN;
        }),
        (t.valueOf = function () {
          return this.toMillis();
        }),
        (t.plus = function (e) {
          if (!this.isValid) return this;
          for (
            var t = m.fromDurationLike(e), n = {}, r = 0, i = V;
            r < i.length;
            r++
          ) {
            var o = i[r];
            (d(t.values, o) || d(this.values, o)) &&
              (n[o] = t.get(o) + this.get(o));
          }
          return E(this, { values: n }, !0);
        }),
        (t.minus = function (e) {
          return this.isValid
            ? ((e = m.fromDurationLike(e)), this.plus(e.negate()))
            : this;
        }),
        (t.mapUnits = function (e) {
          if (!this.isValid) return this;
          for (
            var t = {}, n = 0, r = Object.keys(this.values);
            n < r.length;
            n++
          ) {
            var i = r[n];
            t[i] = Ot(e(this.values[i], i));
          }
          return E(this, { values: t }, !0);
        }),
        (t.get = function (e) {
          return this[m.normalizeUnit(e)];
        }),
        (t.set = function (e) {
          return this.isValid
            ? E(this, { values: l({}, this.values, Tt(e, m.normalizeUnit)) })
            : this;
        }),
        (t.reconfigure = function (e) {
          var e = void 0 === e ? {} : e,
            t = e.locale,
            n = e.conversionAccuracy,
            r = e.matrix,
            t = this.loc.clone({
              locale: t,
              numberingSystem: e.numberingSystem,
            });
          return E(this, { loc: t, matrix: r, conversionAccuracy: n });
        }),
        (t.as = function (e) {
          return this.isValid ? this.shiftTo(e).get(e) : NaN;
        }),
        (t.normalize = function () {
          var e;
          return this.isValid
            ? ((e = this.toObject()),
              Vn(this.matrix, e),
              E(this, { values: e }, !0))
            : this;
        }),
        (t.rescale = function () {
          var e;
          return this.isValid
            ? ((e = ((e) => {
                for (
                  var t = {}, n = 0, r = Object.entries(e);
                  n < r.length;
                  n++
                ) {
                  var i = r[n],
                    o = i[0],
                    i = i[1];
                  0 !== i && (t[o] = i);
                }
                return t;
              })(this.normalize().shiftToAll().toObject())),
              E(this, { values: e }, !0))
            : this;
        }),
        (t.shiftTo = function () {
          for (var e = arguments.length, t = new Array(e), n = 0; n < e; n++)
            t[n] = arguments[n];
          if (!this.isValid) return this;
          if (0 === t.length) return this;
          for (
            var r,
              t = t.map(function (e) {
                return m.normalizeUnit(e);
              }),
              i = {},
              o = {},
              a = this.toObject(),
              s = 0,
              u = V;
            s < u.length;
            s++
          ) {
            var l = u[s];
            if (0 <= t.indexOf(l)) {
              var c,
                f = l,
                d = 0;
              for (c in o) ((d += this.matrix[c][l] * o[c]), (o[c] = 0));
              v(a[l]) && (d += a[l]);
              var h = Math.trunc(d);
              o[l] = (1e3 * d - 1e3 * (i[l] = h)) / 1e3;
            } else v(a[l]) && (o[l] = a[l]);
          }
          for (r in o)
            0 !== o[r] && (i[f] += r === f ? o[r] : o[r] / this.matrix[f][r]);
          return (Vn(this.matrix, i), E(this, { values: i }, !0));
        }),
        (t.shiftToAll = function () {
          return this.isValid
            ? this.shiftTo(
                "years",
                "months",
                "weeks",
                "days",
                "hours",
                "minutes",
                "seconds",
                "milliseconds",
              )
            : this;
        }),
        (t.negate = function () {
          if (!this.isValid) return this;
          for (
            var e = {}, t = 0, n = Object.keys(this.values);
            t < n.length;
            t++
          ) {
            var r = n[t];
            e[r] = 0 === this.values[r] ? 0 : -this.values[r];
          }
          return E(this, { values: e }, !0);
        }),
        (t.equals = function (e) {
          if (!this.isValid || !e.isValid) return !1;
          if (!this.loc.equals(e.loc)) return !1;
          for (var t, n = 0, r = V; n < r.length; n++) {
            var i = r[n];
            if (
              ((t = this.values[i]),
              (i = e.values[i]),
              !(void 0 === t || 0 === t ? void 0 === i || 0 === i : t === i))
            )
              return !1;
          }
          return !0;
        }),
        i(m, [
          {
            key: "locale",
            get: function () {
              return this.isValid ? this.loc.locale : null;
            },
          },
          {
            key: "numberingSystem",
            get: function () {
              return this.isValid ? this.loc.numberingSystem : null;
            },
          },
          {
            key: "years",
            get: function () {
              return this.isValid ? this.values.years || 0 : NaN;
            },
          },
          {
            key: "quarters",
            get: function () {
              return this.isValid ? this.values.quarters || 0 : NaN;
            },
          },
          {
            key: "months",
            get: function () {
              return this.isValid ? this.values.months || 0 : NaN;
            },
          },
          {
            key: "weeks",
            get: function () {
              return this.isValid ? this.values.weeks || 0 : NaN;
            },
          },
          {
            key: "days",
            get: function () {
              return this.isValid ? this.values.days || 0 : NaN;
            },
          },
          {
            key: "hours",
            get: function () {
              return this.isValid ? this.values.hours || 0 : NaN;
            },
          },
          {
            key: "minutes",
            get: function () {
              return this.isValid ? this.values.minutes || 0 : NaN;
            },
          },
          {
            key: "seconds",
            get: function () {
              return this.isValid ? this.values.seconds || 0 : NaN;
            },
          },
          {
            key: "milliseconds",
            get: function () {
              return this.isValid ? this.values.milliseconds || 0 : NaN;
            },
          },
          {
            key: "isValid",
            get: function () {
              return null === this.invalid;
            },
          },
          {
            key: "invalidReason",
            get: function () {
              return this.invalid ? this.invalid.reason : null;
            },
          },
          {
            key: "invalidExplanation",
            get: function () {
              return this.invalid ? this.invalid.explanation : null;
            },
          },
        ]),
        m
      );
    })(Symbol.for("nodejs.util.inspect.custom")),
    En = "Invalid Interval",
    xn = ((e) => {
      function l(e) {
        ((this.s = e.start),
          (this.e = e.end),
          (this.invalid = e.invalid || null),
          (this.isLuxonInterval = !0));
      }
      ((l.invalid = function (e, t) {
        if ((void 0 === t && (t = null), !e))
          throw new u("need to specify a reason the Interval is invalid");
        e = e instanceof f ? e : new f(e, t);
        if (O.throwOnInvalid) throw new Y(e);
        return new l({ invalid: e });
      }),
        (l.fromDateTimes = function (e, t) {
          var n,
            e = wr(e),
            t = wr(t),
            r =
              ((n = t),
              (r = e) && r.isValid
                ? n && n.isValid
                  ? n < r
                    ? xn.invalid(
                        "end before start",
                        "The end of an interval must be after its start, but you had start=" +
                          r.toISO() +
                          " and end=" +
                          n.toISO(),
                      )
                    : null
                  : xn.invalid("missing or invalid end")
                : xn.invalid("missing or invalid start"));
          return null == r ? new l({ start: e, end: t }) : r;
        }),
        (l.after = function (e, t) {
          ((t = x.fromDurationLike(t)), (e = wr(e)));
          return l.fromDateTimes(e, e.plus(t));
        }),
        (l.before = function (e, t) {
          ((t = x.fromDurationLike(t)), (e = wr(e)));
          return l.fromDateTimes(e.minus(t), e);
        }),
        (l.fromISO = function (e, t) {
          var n,
            r,
            i,
            o = (e || "").split("/", 2),
            a = o[0],
            s = o[1];
          if (a && s) {
            try {
              u = (n = W.fromISO(a, t)).isValid;
            } catch (s) {
              u = !1;
            }
            try {
              i = (r = W.fromISO(s, t)).isValid;
            } catch (s) {
              i = !1;
            }
            if (u && i) return l.fromDateTimes(n, r);
            if (u) {
              o = x.fromISO(s, t);
              if (o.isValid) return l.after(n, o);
            } else if (i) {
              var u = x.fromISO(a, t);
              if (u.isValid) return l.before(r, u);
            }
          }
          return l.invalid(
            "unparsable",
            'the input "' + e + "\" can't be parsed as ISO 8601",
          );
        }),
        (l.isInterval = function (e) {
          return (e && e.isLuxonInterval) || !1;
        }));
      var t = l.prototype;
      return (
        (t.length = function (e) {
          return (
            void 0 === e && (e = "milliseconds"),
            this.isValid ? this.toDuration.apply(this, [e]).get(e) : NaN
          );
        }),
        (t.count = function (e, t) {
          var n, r;
          return this.isValid
            ? ((n = this.start.startOf(
                (e = void 0 === e ? "milliseconds" : e),
                t,
              )),
              (r = (r =
                null != t && t.useLocaleWeeks
                  ? this.end.reconfigure({ locale: n.locale })
                  : this.end).startOf(e, t)),
              Math.floor(r.diff(n, e).get(e)) +
                (r.valueOf() !== this.end.valueOf()))
            : NaN;
        }),
        (t.hasSame = function (e) {
          return (
            !!this.isValid &&
            (this.isEmpty() || this.e.minus(1).hasSame(this.s, e))
          );
        }),
        (t.isEmpty = function () {
          return this.s.valueOf() === this.e.valueOf();
        }),
        (t.isAfter = function (e) {
          return !!this.isValid && this.s > e;
        }),
        (t.isBefore = function (e) {
          return !!this.isValid && this.e <= e;
        }),
        (t.contains = function (e) {
          return !!this.isValid && this.s <= e && this.e > e;
        }),
        (t.set = function (e) {
          var e = void 0 === e ? {} : e,
            t = e.start;
          return this.isValid
            ? l.fromDateTimes(t || this.s, e.end || this.e)
            : this;
        }),
        (t.splitAt = function () {
          var t = this;
          if (!this.isValid) return [];
          for (var e = arguments.length, n = new Array(e), r = 0; r < e; r++)
            n[r] = arguments[r];
          for (
            var i = n
                .map(wr)
                .filter(function (e) {
                  return t.contains(e);
                })
                .sort(function (e, t) {
                  return e.toMillis() - t.toMillis();
                }),
              o = [],
              a = this.s,
              s = 0;
            a < this.e;

          ) {
            var u = i[s] || this.e,
              u = +u > +this.e ? this.e : u;
            (o.push(l.fromDateTimes(a, u)), (a = u), (s += 1));
          }
          return o;
        }),
        (t.splitBy = function (e) {
          var t = x.fromDurationLike(e);
          if (!this.isValid || !t.isValid || 0 === t.as("milliseconds"))
            return [];
          for (var n = this.s, r = 1, i = []; n < this.e; ) {
            var o = this.start.plus(
                t.mapUnits(function (e) {
                  return e * r;
                }),
              ),
              o = +o > +this.e ? this.e : o;
            (i.push(l.fromDateTimes(n, o)), (n = o), (r += 1));
          }
          return i;
        }),
        (t.divideEqually = function (e) {
          return this.isValid
            ? this.splitBy(this.length() / e).slice(0, e)
            : [];
        }),
        (t.overlaps = function (e) {
          return this.e > e.s && this.s < e.e;
        }),
        (t.abutsStart = function (e) {
          return !!this.isValid && +this.e == +e.s;
        }),
        (t.abutsEnd = function (e) {
          return !!this.isValid && +e.e == +this.s;
        }),
        (t.engulfs = function (e) {
          return !!this.isValid && this.s <= e.s && this.e >= e.e;
        }),
        (t.equals = function (e) {
          return (
            !(!this.isValid || !e.isValid) &&
            this.s.equals(e.s) &&
            this.e.equals(e.e)
          );
        }),
        (t.intersection = function (e) {
          var t;
          return this.isValid
            ? ((t = (this.s > e.s ? this : e).s),
              (e = (this.e < e.e ? this : e).e) <= t
                ? null
                : l.fromDateTimes(t, e))
            : this;
        }),
        (t.union = function (e) {
          var t;
          return this.isValid
            ? ((t = (this.s < e.s ? this : e).s),
              (e = (this.e > e.e ? this : e).e),
              l.fromDateTimes(t, e))
            : this;
        }),
        (l.merge = function (e) {
          var e = e
              .sort(function (e, t) {
                return e.s - t.s;
              })
              .reduce(
                function (e, t) {
                  var n = e[0],
                    e = e[1];
                  return e
                    ? e.overlaps(t) || e.abutsStart(t)
                      ? [n, e.union(t)]
                      : [n.concat([e]), t]
                    : [n, t];
                },
                [[], null],
              ),
            t = e[0],
            e = e[1];
          return (e && t.push(e), t);
        }),
        (l.xor = function (e) {
          for (
            var t,
              n = null,
              r = 0,
              i = [],
              e = e.map(function (e) {
                return [
                  { time: e.s, type: "s" },
                  { time: e.e, type: "e" },
                ];
              }),
              o = R(
                (t = Array.prototype).concat.apply(t, e).sort(function (e, t) {
                  return e.time - t.time;
                }),
              );
            !(a = o()).done;

          )
            var a = a.value,
              n =
                1 === (r += "s" === a.type ? 1 : -1)
                  ? a.time
                  : (n && +n != +a.time && i.push(l.fromDateTimes(n, a.time)),
                    null);
          return l.merge(i);
        }),
        (t.difference = function () {
          for (
            var t = this, e = arguments.length, n = new Array(e), r = 0;
            r < e;
            r++
          )
            n[r] = arguments[r];
          return l
            .xor([this].concat(n))
            .map(function (e) {
              return t.intersection(e);
            })
            .filter(function (e) {
              return e && !e.isEmpty();
            });
        }),
        (t.toString = function () {
          return this.isValid
            ? "[" + this.s.toISO() + " – " + this.e.toISO() + ")"
            : En;
        }),
        (t[e] = function () {
          return this.isValid
            ? "Interval { start: " +
                this.s.toISO() +
                ", end: " +
                this.e.toISO() +
                " }"
            : "Interval { Invalid, reason: " + this.invalidReason + " }";
        }),
        (t.toLocaleString = function (e, t) {
          return (
            void 0 === e && (e = G),
            void 0 === t && (t = {}),
            this.isValid
              ? p.create(this.s.loc.clone(t), e).formatInterval(this)
              : En
          );
        }),
        (t.toISO = function (e) {
          return this.isValid ? this.s.toISO(e) + "/" + this.e.toISO(e) : En;
        }),
        (t.toISODate = function () {
          return this.isValid
            ? this.s.toISODate() + "/" + this.e.toISODate()
            : En;
        }),
        (t.toISOTime = function (e) {
          return this.isValid
            ? this.s.toISOTime(e) + "/" + this.e.toISOTime(e)
            : En;
        }),
        (t.toFormat = function (e, t) {
          ((t = (void 0 === t ? {} : t).separator),
            (t = void 0 === t ? " – " : t));
          return this.isValid
            ? "" + this.s.toFormat(e) + t + this.e.toFormat(e)
            : En;
        }),
        (t.toDuration = function (e, t) {
          return this.isValid
            ? this.e.diff(this.s, e, t)
            : x.invalid(this.invalidReason);
        }),
        (t.mapEndpoints = function (e) {
          return l.fromDateTimes(e(this.s), e(this.e));
        }),
        i(l, [
          {
            key: "start",
            get: function () {
              return this.isValid ? this.s : null;
            },
          },
          {
            key: "end",
            get: function () {
              return this.isValid ? this.e : null;
            },
          },
          {
            key: "isValid",
            get: function () {
              return null === this.invalidReason;
            },
          },
          {
            key: "invalidReason",
            get: function () {
              return this.invalid ? this.invalid.reason : null;
            },
          },
          {
            key: "invalidExplanation",
            get: function () {
              return this.invalid ? this.invalid.explanation : null;
            },
          },
        ]),
        l
      );
    })(Symbol.for("nodejs.util.inspect.custom")),
    Fn = (() => {
      function e() {}
      return (
        (e.hasDST = function (e) {
          void 0 === e && (e = O.defaultZone);
          var t = W.now().setZone(e).set({ month: 12 });
          return !e.isUniversal && t.offset !== t.set({ month: 6 }).offset;
        }),
        (e.isValidIANAZone = function (e) {
          return a.isValidZone(e);
        }),
        (e.normalizeZone = function (e) {
          return S(e, O.defaultZone);
        }),
        (e.getStartOfWeek = function (e) {
          var e = void 0 === e ? {} : e,
            t = e.locale,
            e = e.locObj;
          return (
            (void 0 === e ? null : e) || b.create(void 0 === t ? null : t)
          ).getStartOfWeek();
        }),
        (e.getMinimumDaysInFirstWeek = function (e) {
          var e = void 0 === e ? {} : e,
            t = e.locale,
            e = e.locObj;
          return (
            (void 0 === e ? null : e) || b.create(void 0 === t ? null : t)
          ).getMinDaysInFirstWeek();
        }),
        (e.getWeekendWeekdays = function (e) {
          var e = void 0 === e ? {} : e,
            t = e.locale,
            e = e.locObj;
          return (
            (void 0 === e ? null : e) || b.create(void 0 === t ? null : t)
          )
            .getWeekendDays()
            .slice();
        }),
        (e.months = function (e, t) {
          void 0 === e && (e = "long");
          var t = void 0 === t ? {} : t,
            n = t.locale,
            r = t.numberingSystem,
            i = t.locObj,
            i = void 0 === i ? null : i,
            t = t.outputCalendar;
          return (
            i ||
            b.create(
              void 0 === n ? null : n,
              void 0 === r ? null : r,
              void 0 === t ? "gregory" : t,
            )
          ).months(e);
        }),
        (e.monthsFormat = function (e, t) {
          void 0 === e && (e = "long");
          var t = void 0 === t ? {} : t,
            n = t.locale,
            r = t.numberingSystem,
            i = t.locObj,
            i = void 0 === i ? null : i,
            t = t.outputCalendar;
          return (
            i ||
            b.create(
              void 0 === n ? null : n,
              void 0 === r ? null : r,
              void 0 === t ? "gregory" : t,
            )
          ).months(e, !0);
        }),
        (e.weekdays = function (e, t) {
          void 0 === e && (e = "long");
          var t = void 0 === t ? {} : t,
            n = t.locale,
            r = t.numberingSystem,
            t = t.locObj;
          return (
            (void 0 === t ? null : t) ||
            b.create(void 0 === n ? null : n, void 0 === r ? null : r, null)
          ).weekdays(e);
        }),
        (e.weekdaysFormat = function (e, t) {
          void 0 === e && (e = "long");
          var t = void 0 === t ? {} : t,
            n = t.locale,
            r = t.numberingSystem,
            t = t.locObj;
          return (
            (void 0 === t ? null : t) ||
            b.create(void 0 === n ? null : n, void 0 === r ? null : r, null)
          ).weekdays(e, !0);
        }),
        (e.meridiems = function (e) {
          e = (void 0 === e ? {} : e).locale;
          return b.create(void 0 === e ? null : e).meridiems();
        }),
        (e.eras = function (e, t) {
          void 0 === e && (e = "short");
          t = (void 0 === t ? {} : t).locale;
          return b.create(void 0 === t ? null : t, null, "gregory").eras(e);
        }),
        (e.features = function () {
          return { relative: ut(), localeWeek: lt() };
        }),
        e
      );
    })();
  function Cn(e, t) {
    function n(e) {
      return e.toUTC(0, { keepLocalTime: !0 }).startOf("day").valueOf();
    }
    t = n(t) - n(e);
    return Math.floor(x.fromMillis(t).as("days"));
  }
  function Zn(e, t, n, r) {
    var e = ((e, t, n) => {
        for (
          var r,
            i,
            o = {},
            a = e,
            s = 0,
            u = [
              [
                "years",
                function (e, t) {
                  return t.year - e.year;
                },
              ],
              [
                "quarters",
                function (e, t) {
                  return t.quarter - e.quarter + 4 * (t.year - e.year);
                },
              ],
              [
                "months",
                function (e, t) {
                  return t.month - e.month + 12 * (t.year - e.year);
                },
              ],
              [
                "weeks",
                function (e, t) {
                  e = Cn(e, t);
                  return (e - (e % 7)) / 7;
                },
              ],
              ["days", Cn],
            ];
          s < u.length;
          s++
        ) {
          var l = u[s],
            c = l[0],
            l = l[1];
          0 <= n.indexOf(c) &&
            ((o[(r = c)] = l(e, t)),
            t < (i = a.plus(o))
              ? (o[c]--,
                t < (e = a.plus(o)) && ((i = e), o[c]--, (e = a.plus(o))))
              : (e = i));
        }
        return [e, o, i, r];
      })(e, t, n),
      i = e[0],
      o = e[1],
      a = e[2],
      e = e[3],
      s = t - i,
      n = n.filter(function (e) {
        return 0 <= ["hours", "minutes", "seconds", "milliseconds"].indexOf(e);
      }),
      t =
        (0 === n.length &&
          (a = a < t ? i.plus((((t = {})[e] = 1), t)) : a) !== i &&
          (o[e] = (o[e] || 0) + s / (a - i)),
        x.fromObject(o, r));
    return 0 < n.length
      ? (e = x.fromMillis(s, r)).shiftTo.apply(e, n).plus(t)
      : t;
  }
  var Wn = "missing Intl.DateTimeFormat.formatToParts support";
  function F(e, t) {
    return (
      void 0 === t &&
        (t = function (e) {
          return e;
        }),
      {
        regex: e,
        deser: function (e) {
          e = e[0];
          return t(
            ((e) => {
              var t = parseInt(e, 10);
              if (isNaN(t)) {
                for (var t = "", n = 0; n < e.length; n++) {
                  var r = e.charCodeAt(n);
                  if (-1 !== e[n].search(je.hanidec)) t += Ae.indexOf(e[n]);
                  else
                    for (var i in ze) {
                      var i = ze[i],
                        o = i[0];
                      o <= r && r <= i[1] && (t += r - o);
                    }
                }
                return parseInt(t, 10);
              }
              return t;
            })(e),
          );
        },
      }
    );
  }
  var Ln = "[ " + String.fromCharCode(160) + "]",
    jn = new RegExp(Ln, "g");
  function zn(e) {
    return e.replace(/\./g, "\\.?").replace(jn, Ln);
  }
  function An(e) {
    return e.replace(/\./g, "").replace(jn, " ").toLowerCase();
  }
  function C(n, r) {
    return null === n
      ? null
      : {
          regex: RegExp(n.map(zn).join("|")),
          deser: function (e) {
            var t = e[0];
            return (
              n.findIndex(function (e) {
                return An(t) === An(e);
              }) + r
            );
          },
        };
  }
  function qn(e, t) {
    return {
      regex: e,
      deser: function (e) {
        return St(e[1], e[2]);
      },
      groups: t,
    };
  }
  function _n(e) {
    return {
      regex: e,
      deser: function (e) {
        return e[0];
      },
    };
  }
  function Un(t, n) {
    function r(e) {
      return {
        regex: RegExp(e.val.replace(/[\-\[\]{}()*+?.,\\\^$|#\s]/g, "\\$&")),
        deser: function (e) {
          return e[0];
        },
        literal: !0,
      };
    }
    var i = y(n),
      o = y(n, "{2}"),
      a = y(n, "{3}"),
      s = y(n, "{4}"),
      u = y(n, "{6}"),
      l = y(n, "{1,2}"),
      c = y(n, "{1,3}"),
      f = y(n, "{1,6}"),
      d = y(n, "{1,9}"),
      h = y(n, "{2,4}"),
      m = y(n, "{4,6}"),
      e = ((e) => {
        if (t.literal) return r(e);
        switch (e.val) {
          case "G":
            return C(n.eras("short"), 0);
          case "GG":
            return C(n.eras("long"), 0);
          case "y":
            return F(f);
          case "yy":
            return F(h, wt);
          case "yyyy":
            return F(s);
          case "yyyyy":
            return F(m);
          case "yyyyyy":
            return F(u);
          case "M":
            return F(l);
          case "MM":
            return F(o);
          case "MMM":
            return C(n.months("short", !0), 1);
          case "MMMM":
            return C(n.months("long", !0), 1);
          case "L":
            return F(l);
          case "LL":
            return F(o);
          case "LLL":
            return C(n.months("short", !1), 1);
          case "LLLL":
            return C(n.months("long", !1), 1);
          case "d":
            return F(l);
          case "dd":
            return F(o);
          case "o":
            return F(c);
          case "ooo":
            return F(a);
          case "HH":
            return F(o);
          case "H":
            return F(l);
          case "hh":
            return F(o);
          case "h":
            return F(l);
          case "mm":
            return F(o);
          case "m":
          case "q":
            return F(l);
          case "qq":
            return F(o);
          case "s":
            return F(l);
          case "ss":
            return F(o);
          case "S":
            return F(c);
          case "SSS":
            return F(a);
          case "u":
            return _n(d);
          case "uu":
            return _n(l);
          case "uuu":
            return F(i);
          case "a":
            return C(n.meridiems(), 0);
          case "kkkk":
            return F(s);
          case "kk":
            return F(h, wt);
          case "W":
            return F(l);
          case "WW":
            return F(o);
          case "E":
          case "c":
            return F(i);
          case "EEE":
            return C(n.weekdays("short", !1), 1);
          case "EEEE":
            return C(n.weekdays("long", !1), 1);
          case "ccc":
            return C(n.weekdays("short", !0), 1);
          case "cccc":
            return C(n.weekdays("long", !0), 1);
          case "Z":
          case "ZZ":
            return qn(
              new RegExp("([+-]" + l.source + ")(?::(" + o.source + "))?"),
              2,
            );
          case "ZZZ":
            return qn(
              new RegExp("([+-]" + l.source + ")(" + o.source + ")?"),
              2,
            );
          case "z":
            return _n(/[a-z_+-/]{1,256}?/i);
          case " ":
            return _n(/[^\S\n\r]/);
          default:
            return r(e);
        }
      })(t) || { invalidReason: Wn };
    return ((e.token = t), e);
  }
  var Rn = {
    year: { "2-digit": "yy", numeric: "yyyyy" },
    month: { numeric: "M", "2-digit": "MM", short: "MMM", long: "MMMM" },
    day: { numeric: "d", "2-digit": "dd" },
    weekday: { short: "EEE", long: "EEEE" },
    dayperiod: "a",
    dayPeriod: "a",
    hour12: { numeric: "h", "2-digit": "hh" },
    hour24: { numeric: "H", "2-digit": "HH" },
    minute: { numeric: "m", "2-digit": "mm" },
    second: { numeric: "s", "2-digit": "ss" },
    timeZoneName: { long: "ZZZZZ", short: "ZZZ" },
  };
  function Pn(r) {
    var e,
      t = null;
    return (
      N(r.z) || (t = a.create(r.z)),
      N(r.Z) || ((t = t || new c(r.Z)), (e = r.Z)),
      N(r.q) || (r.M = 3 * (r.q - 1) + 1),
      N(r.h) ||
        (r.h < 12 && 1 === r.a
          ? (r.h += 12)
          : 12 === r.h && 0 === r.a && (r.h = 0)),
      0 === r.G && r.y && (r.y = -r.y),
      N(r.u) || (r.S = dt(r.u)),
      [
        Object.keys(r).reduce(function (e, t) {
          var n = ((e) => {
            switch (e) {
              case "S":
                return "millisecond";
              case "s":
                return "second";
              case "m":
                return "minute";
              case "h":
              case "H":
                return "hour";
              case "d":
                return "day";
              case "o":
                return "ordinal";
              case "L":
              case "M":
                return "month";
              case "y":
                return "year";
              case "E":
              case "c":
                return "weekday";
              case "W":
                return "weekNumber";
              case "k":
                return "weekYear";
              case "q":
                return "quarter";
              default:
                return null;
            }
          })(t);
          return (n && (e[n] = r[t]), e);
        }, {}),
        t,
        e,
      ]
    );
  }
  var Yn = null;
  function Hn(e, n) {
    var t;
    return (t = Array.prototype).concat.apply(
      t,
      e.map(function (e) {
        return (
          (t = n),
          (e = e).literal ||
          null == (t = $n(p.macroTokenToFormatOpts(e.val), t)) ||
          t.includes(void 0)
            ? e
            : t
        );
        var t;
      }),
    );
  }
  var Jn = (() => {
    function e(t, e) {
      var n;
      ((this.locale = t),
        (this.format = e),
        (this.tokens = Hn(p.parseFormat(e), t)),
        (this.units = this.tokens.map(function (e) {
          return Un(e, t);
        })),
        (this.disqualifyingUnit = this.units.find(function (e) {
          return e.invalidReason;
        })),
        this.disqualifyingUnit ||
          ((n = (e = [
            "^" +
              (e = this.units)
                .map(function (e) {
                  return e.regex;
                })
                .reduce(function (e, t) {
                  return e + "(" + t.source + ")";
                }, "") +
              "$",
            e,
          ])[1]),
          (this.regex = RegExp(e[0], "i")),
          (this.handlers = n)));
    }
    return (
      (e.prototype.explainFromTokens = function (e) {
        if (this.isValid) {
          var t = ((e, t, n) => {
              var r = e.match(t);
              if (r) {
                var i,
                  o,
                  a,
                  s = {},
                  u = 1;
                for (i in n)
                  d(n, i) &&
                    ((a = (o = n[i]).groups ? o.groups + 1 : 1),
                    !o.literal &&
                      o.token &&
                      (s[o.token.val[0]] = o.deser(r.slice(u, u + a))),
                    (u += a));
                return [r, s];
              }
              return [r, {}];
            })(e, this.regex, this.handlers),
            n = t[0],
            t = t[1],
            r = t ? Pn(t) : [null, null, void 0],
            i = r[0],
            o = r[1],
            r = r[2];
          if (d(t, "a") && d(t, "H"))
            throw new w(
              "Can't include meridiem when specifying 24-hour format",
            );
          return {
            input: e,
            tokens: this.tokens,
            regex: this.regex,
            rawMatches: n,
            matches: t,
            result: i,
            zone: o,
            specificOffset: r,
          };
        }
        return {
          input: e,
          tokens: this.tokens,
          invalidReason: this.invalidReason,
        };
      }),
      i(e, [
        {
          key: "isValid",
          get: function () {
            return !this.disqualifyingUnit;
          },
        },
        {
          key: "invalidReason",
          get: function () {
            return this.disqualifyingUnit
              ? this.disqualifyingUnit.invalidReason
              : null;
          },
        },
      ]),
      e
    );
  })();
  function Gn(e, t, n) {
    return new Jn(e, n).explainFromTokens(t);
  }
  function $n(o, e) {
    var t, a;
    return o
      ? ((t = (e = p
          .create(e, o)
          .dtFormatter(
            (Yn = Yn || W.fromMillis(1555555555555)),
          )).formatToParts()),
        (a = e.resolvedOptions()),
        t.map(function (e) {
          return (
            (t = o),
            (n = a),
            (i = (e = e).type),
            (e = e.value),
            "literal" === i
              ? { literal: !(r = /^\s+$/.test(e)), val: r ? " " : e }
              : ((r = t[i]),
                "hour" === (e = i) &&
                  (e =
                    null != t.hour12
                      ? t.hour12
                        ? "hour12"
                        : "hour24"
                      : null != t.hourCycle
                        ? "h11" === t.hourCycle || "h12" === t.hourCycle
                          ? "hour12"
                          : "hour24"
                        : n.hour12
                          ? "hour12"
                          : "hour24"),
                (i = "object" == typeof (i = Rn[e]) ? i[r] : i)
                  ? { literal: !1, val: i }
                  : void 0)
          );
          var t, n, r, i;
        }))
      : null;
  }
  var Bn = "Invalid DateTime";
  function Qn(e) {
    return new f(
      "unsupported zone",
      'the zone "' + e.name + '" is not supported',
    );
  }
  function Kn(e) {
    return (null === e.weekData && (e.weekData = et(e.c)), e.weekData);
  }
  function Xn(e) {
    return (
      null === e.localWeekData &&
        (e.localWeekData = et(
          e.c,
          e.loc.getMinDaysInFirstWeek(),
          e.loc.getStartOfWeek(),
        )),
      e.localWeekData
    );
  }
  function Z(e, t) {
    e = {
      ts: e.ts,
      zone: e.zone,
      c: e.c,
      o: e.o,
      loc: e.loc,
      invalid: e.invalid,
    };
    return new W(l({}, e, t, { old: e }));
  }
  function er(e, t, n) {
    var r = e - 60 * t * 1e3,
      i = n.offset(r);
    return t === i
      ? [r, t]
      : i === (n = n.offset((r -= 60 * (i - t) * 1e3)))
        ? [r, i]
        : [e - 60 * Math.min(i, n) * 1e3, Math.max(i, n)];
  }
  function tr(e, t) {
    e += 60 * t * 1e3;
    t = new Date(e);
    return {
      year: t.getUTCFullYear(),
      month: t.getUTCMonth() + 1,
      day: t.getUTCDate(),
      hour: t.getUTCHours(),
      minute: t.getUTCMinutes(),
      second: t.getUTCSeconds(),
      millisecond: t.getUTCMilliseconds(),
    };
  }
  function nr(e, t, n) {
    return er(gt(e), t, n);
  }
  function rr(e, t) {
    var n = e.o,
      r = e.c.year + Math.trunc(t.years),
      i = e.c.month + Math.trunc(t.months) + 3 * Math.trunc(t.quarters),
      r = l({}, e.c, {
        year: r,
        month: i,
        day:
          Math.min(e.c.day, vt(r, i)) +
          Math.trunc(t.days) +
          7 * Math.trunc(t.weeks),
      }),
      i = x
        .fromObject({
          years: t.years - Math.trunc(t.years),
          quarters: t.quarters - Math.trunc(t.quarters),
          months: t.months - Math.trunc(t.months),
          weeks: t.weeks - Math.trunc(t.weeks),
          days: t.days - Math.trunc(t.days),
          hours: t.hours,
          minutes: t.minutes,
          seconds: t.seconds,
          milliseconds: t.milliseconds,
        })
        .as("milliseconds"),
      t = er(gt(r), n, e.zone),
      r = t[0],
      n = t[1];
    return (0 !== i && (n = e.zone.offset((r += i))), { ts: r, o: n });
  }
  function ir(e, t, n, r, i, o) {
    var a = n.setZone,
      s = n.zone;
    return (e && 0 !== Object.keys(e).length) || t
      ? ((e = W.fromObject(e, l({}, n, { zone: t || s, specificOffset: o }))),
        a ? e : e.setZone(s))
      : W.invalid(
          new f("unparsable", 'the input "' + i + "\" can't be parsed as " + r),
        );
  }
  function or(e, t, n) {
    return (
      void 0 === n && (n = !0),
      e.isValid
        ? p
            .create(b.create("en-US"), { allowZ: n, forceSimple: !0 })
            .formatDateTimeFromString(e, t)
        : null
    );
  }
  function ar(e, t) {
    var n = 9999 < e.c.year || e.c.year < 0,
      r = "";
    return (
      n && 0 <= e.c.year && (r += "+"),
      (r += h(e.c.year, n ? 6 : 4)),
      (r = t
        ? (r = (r += "-") + h(e.c.month) + "-") + h(e.c.day)
        : (r += h(e.c.month)) + h(e.c.day))
    );
  }
  function sr(e, t, n, r, i, o) {
    var a = h(e.c.hour);
    return (
      t
        ? ((a = (a += ":") + h(e.c.minute)),
          (0 === e.c.millisecond && 0 === e.c.second && n) || (a += ":"))
        : (a += h(e.c.minute)),
      (0 === e.c.millisecond && 0 === e.c.second && n) ||
        ((a += h(e.c.second)), 0 === e.c.millisecond && r) ||
        (a = (a += ".") + h(e.c.millisecond, 3)),
      i &&
        (e.isOffsetFixed && 0 === e.offset && !o
          ? (a += "Z")
          : (a =
              e.o < 0
                ? (a = (a += "-") + h(Math.trunc(-e.o / 60)) + ":") +
                  h(Math.trunc(-e.o % 60))
                : (a = (a += "+") + h(Math.trunc(e.o / 60)) + ":") +
                  h(Math.trunc(e.o % 60)))),
      o && (a += "[" + e.zone.ianaName + "]"),
      a
    );
  }
  var ur,
    lr = { month: 1, day: 1, hour: 0, minute: 0, second: 0, millisecond: 0 },
    cr = {
      weekNumber: 1,
      weekday: 1,
      hour: 0,
      minute: 0,
      second: 0,
      millisecond: 0,
    },
    fr = { ordinal: 1, hour: 0, minute: 0, second: 0, millisecond: 0 },
    dr = ["year", "month", "day", "hour", "minute", "second", "millisecond"],
    hr = [
      "weekYear",
      "weekNumber",
      "weekday",
      "hour",
      "minute",
      "second",
      "millisecond",
    ],
    mr = ["year", "ordinal", "hour", "minute", "second", "millisecond"];
  function yr(e) {
    switch (e.toLowerCase()) {
      case "localweekday":
      case "localweekdays":
        return "localWeekday";
      case "localweeknumber":
      case "localweeknumbers":
        return "localWeekNumber";
      case "localweekyear":
      case "localweekyears":
        return "localWeekYear";
      default:
        var t = e,
          n = {
            year: "year",
            years: "year",
            month: "month",
            months: "month",
            day: "day",
            days: "day",
            hour: "hour",
            hours: "hour",
            minute: "minute",
            minutes: "minute",
            quarter: "quarter",
            quarters: "quarter",
            second: "second",
            seconds: "second",
            millisecond: "millisecond",
            milliseconds: "millisecond",
            weekday: "weekday",
            weekdays: "weekday",
            weeknumber: "weekNumber",
            weeksnumber: "weekNumber",
            weeknumbers: "weekNumber",
            weekyear: "weekYear",
            weekyears: "weekYear",
            ordinal: "ordinal",
          }[t.toLowerCase()];
        if (n) return n;
        throw new J(t);
    }
  }
  function vr(e, t) {
    var n = S(t.zone, O.defaultZone);
    if (!n.isValid) return W.invalid(Qn(n));
    t = b.fromObject(t);
    if (N(e.year)) s = O.now();
    else {
      for (var r = 0, i = dr; r < i.length; r++) {
        var o = i[r];
        N(e[o]) && (e[o] = lr[o]);
      }
      var a = ot(e) || at(e);
      if (a) return W.invalid(a);
      kr[(a = n)] || (void 0 === ur && (ur = O.now()), (kr[a] = a.offset(ur)));
      var a = nr(e, kr[a], n),
        s = a[0],
        a = a[1];
    }
    return new W({ ts: s, zone: n, loc: t, o: a });
  }
  function gr(t, n, r) {
    function e(e, t) {
      return (
        (e = ht(e, o || r.calendary ? 0 : 2, !0)),
        n.loc.clone(r).relFormatter(r).format(e, t)
      );
    }
    function i(e) {
      return r.calendary
        ? n.hasSame(t, e)
          ? 0
          : n.startOf(e).diff(t.startOf(e), e).get(e)
        : n.diff(t, e).get(e);
    }
    var o = !!N(r.round) || r.round;
    if (r.unit) return e(i(r.unit), r.unit);
    for (var a = R(r.units); !(s = a()).done; ) {
      var s = s.value,
        u = i(s);
      if (1 <= Math.abs(u)) return e(u, s);
    }
    return e(n < t ? -0 : 0, r.units[r.units.length - 1]);
  }
  function pr(e) {
    var t = {},
      e =
        0 < e.length && "object" == typeof e[e.length - 1]
          ? ((t = e[e.length - 1]), Array.from(e).slice(0, e.length - 1))
          : Array.from(e);
    return [t, e];
  }
  var kr = {},
    W = ((e) => {
      function k(e) {
        var t,
          n = e.zone || O.defaultZone,
          r =
            e.invalid ||
            (Number.isNaN(e.ts) ? new f("invalid input") : null) ||
            (n.isValid ? null : Qn(n)),
          i = ((this.ts = N(e.ts) ? O.now() : e.ts), null),
          o = null;
        (r ||
          (o =
            e.old && e.old.ts === this.ts && e.old.zone.equals(n)
              ? ((i = (t = [e.old.c, e.old.o])[0]), t[1])
              : ((t = v(e.o) && !e.old ? e.o : n.offset(this.ts)),
                (i = tr(this.ts, t)),
                (i = (r = Number.isNaN(i.year) ? new f("invalid input") : null)
                  ? null
                  : i),
                r ? null : t)),
          (this._zone = n),
          (this.loc = e.loc || b.create()),
          (this.invalid = r),
          (this.weekData = null),
          (this.localWeekData = null),
          (this.c = i),
          (this.o = o),
          (this.isLuxonDateTime = !0));
      }
      ((k.now = function () {
        return new k({});
      }),
        (k.local = function () {
          var e = pr(arguments),
            t = e[0],
            e = e[1];
          return vr(
            {
              year: e[0],
              month: e[1],
              day: e[2],
              hour: e[3],
              minute: e[4],
              second: e[5],
              millisecond: e[6],
            },
            t,
          );
        }),
        (k.utc = function () {
          var e = pr(arguments),
            t = e[0],
            e = e[1],
            n = e[0],
            r = e[1],
            i = e[2],
            o = e[3],
            a = e[4],
            s = e[5],
            e = e[6];
          return (
            (t.zone = c.utcInstance),
            vr(
              {
                year: n,
                month: r,
                day: i,
                hour: o,
                minute: a,
                second: s,
                millisecond: e,
              },
              t,
            )
          );
        }),
        (k.fromJSDate = function (e, t) {
          void 0 === t && (t = {});
          var n,
            e =
              "[object Date]" === Object.prototype.toString.call(e)
                ? e.valueOf()
                : NaN;
          return Number.isNaN(e)
            ? k.invalid("invalid input")
            : (n = S(t.zone, O.defaultZone)).isValid
              ? new k({ ts: e, zone: n, loc: b.fromObject(t) })
              : k.invalid(Qn(n));
        }),
        (k.fromMillis = function (e, t) {
          if ((void 0 === t && (t = {}), v(e)))
            return e < -864e13 || 864e13 < e
              ? k.invalid("Timestamp out of range")
              : new k({
                  ts: e,
                  zone: S(t.zone, O.defaultZone),
                  loc: b.fromObject(t),
                });
          throw new u(
            "fromMillis requires a numerical input, but received a " +
              typeof e +
              " with value " +
              e,
          );
        }),
        (k.fromSeconds = function (e, t) {
          if ((void 0 === t && (t = {}), v(e)))
            return new k({
              ts: 1e3 * e,
              zone: S(t.zone, O.defaultZone),
              loc: b.fromObject(t),
            });
          throw new u("fromSeconds requires a numerical input");
        }),
        (k.fromObject = function (e, t) {
          e = e || {};
          var n = S((t = void 0 === t ? {} : t).zone, O.defaultZone);
          if (!n.isValid) return k.invalid(Qn(n));
          var r = b.fromObject(t),
            i = Tt(e, yr),
            o = it(i, r),
            a = o.minDaysInFirstWeek,
            o = o.startOfWeek,
            s = O.now(),
            t = N(t.specificOffset) ? n.offset(s) : t.specificOffset,
            u = !N(i.ordinal),
            l = !N(i.year),
            c = !N(i.month) || !N(i.day),
            l = l || c,
            f = i.weekYear || i.weekNumber;
          if ((l || u) && f)
            throw new w(
              "Can't mix weekYear/weekNumber units with year/month/day or ordinals",
            );
          if (c && u) throw new w("Can't mix ordinal dates with month/day");
          for (
            var d,
              c = f || (i.weekday && !l),
              h = tr(s, t),
              m =
                (c
                  ? ((p = hr), (d = cr), (h = et(h, a, o)))
                  : u
                    ? ((p = mr), (d = fr), (h = nt(h)))
                    : ((p = dr), (d = lr)),
                !1),
              y = R(p);
            !(v = y()).done;

          ) {
            var v = v.value;
            N(i[v]) ? (i[v] = (m ? d : h)[v]) : (m = !0);
          }
          var g,
            p =
              (c
                ? ((f = a),
                  (s = o),
                  (g = st((p = i).weekYear)),
                  (f = D(
                    p.weekNumber,
                    1,
                    kt(
                      p.weekYear,
                      (f = void 0 === f ? 4 : f),
                      (s = void 0 === s ? 1 : s),
                    ),
                  )),
                  (s = D(p.weekday, 1, 7)),
                  g
                    ? f
                      ? !s && T("weekday", p.weekday)
                      : T("week", p.weekNumber)
                    : T("weekYear", p.weekYear))
                : u
                  ? ((f = st((g = i).year)),
                    (s = D(g.ordinal, 1, yt(g.year))),
                    f ? !s && T("ordinal", g.ordinal) : T("year", g.year))
                  : ot(i)) || at(i);
          return p
            ? k.invalid(p)
            : ((s = new k({
                ts: (f = nr(c ? tt(i, a, o) : u ? rt(i) : i, t, n))[0],
                zone: n,
                o: f[1],
                loc: r,
              })),
              i.weekday && l && e.weekday !== s.weekday
                ? k.invalid(
                    "mismatched weekday",
                    "you can't specify both a weekday of " +
                      i.weekday +
                      " and a date of " +
                      s.toISO(),
                  )
                : s.isValid
                  ? s
                  : k.invalid(s.invalid));
        }),
        (k.fromISO = function (e, t) {
          void 0 === t && (t = {});
          var n = Pt(e, [dn, vn], [hn, gn], [mn, pn], [yn, kn]);
          return ir(n[0], n[1], t, "ISO 8601", e);
        }),
        (k.fromRFC2822 = function (e, t) {
          void 0 === t && (t = {});
          var n = Pt(
            e
              .replace(/\([^()]*\)|[\n\t]/g, " ")
              .replace(/(\s\s+)/g, " ")
              .trim(),
            [on, an],
          );
          return ir(n[0], n[1], t, "RFC 2822", e);
        }),
        (k.fromHTTP = function (e, t) {
          void 0 === t && (t = {});
          e = Pt(e, [sn, cn], [un, cn], [ln, fn]);
          return ir(e[0], e[1], t, "HTTP", t);
        }),
        (k.fromFormat = function (e, t, n) {
          if ((void 0 === n && (n = {}), N(e) || N(t)))
            throw new u("fromFormat requires an input string and a format");
          var r = n,
            i = r.locale,
            r = r.numberingSystem,
            i = b.fromOpts({
              locale: void 0 === i ? null : i,
              numberingSystem: void 0 === r ? null : r,
              defaultToEN: !0,
            }),
            i = [
              (r = Gn((r = i), e, t)).result,
              r.zone,
              r.specificOffset,
              r.invalidReason,
            ],
            r = i[0],
            o = i[1],
            a = i[2],
            i = i[3];
          return i ? k.invalid(i) : ir(r, o, n, "format " + t, e, a);
        }),
        (k.fromString = function (e, t, n) {
          return k.fromFormat(e, t, (n = void 0 === n ? {} : n));
        }),
        (k.fromSQL = function (e, t) {
          void 0 === t && (t = {});
          var n = Pt(e, [bn, vn], [Sn, On]);
          return ir(n[0], n[1], t, "SQL", e);
        }),
        (k.invalid = function (e, t) {
          if ((void 0 === t && (t = null), !e))
            throw new u("need to specify a reason the DateTime is invalid");
          e = e instanceof f ? e : new f(e, t);
          if (O.throwOnInvalid) throw new P(e);
          return new k({ invalid: e });
        }),
        (k.isDateTime = function (e) {
          return (e && e.isLuxonDateTime) || !1;
        }),
        (k.parseFormatForOpts = function (e, t) {
          e = $n(e, b.fromObject((t = void 0 === t ? {} : t)));
          return e
            ? e
                .map(function (e) {
                  return e ? e.val : null;
                })
                .join("")
            : null;
        }),
        (k.expandFormat = function (e, t) {
          return (
            void 0 === t && (t = {}),
            Hn(p.parseFormat(e), b.fromObject(t))
              .map(function (e) {
                return e.val;
              })
              .join("")
          );
        }),
        (k.resetCache = function () {
          ((ur = void 0), (kr = {}));
        }));
      var t = k.prototype;
      return (
        (t.get = function (e) {
          return this[e];
        }),
        (t.getPossibleOffsets = function () {
          var e, t, n, r;
          return this.isValid &&
            !this.isOffsetFixed &&
            ((e = gt(this.c)),
            (n = this.zone.offset(e - 864e5)),
            (r = this.zone.offset(e + 864e5)),
            (n = this.zone.offset(e - 6e4 * n)) !==
              (r = this.zone.offset(e - 6e4 * r))) &&
            ((t = e - 6e4 * r),
            (n = tr((e = e - 6e4 * n), n)),
            (r = tr(t, r)),
            n.hour === r.hour) &&
            n.minute === r.minute &&
            n.second === r.second &&
            n.millisecond === r.millisecond
            ? [Z(this, { ts: e }), Z(this, { ts: t })]
            : [this];
        }),
        (t.resolvedLocaleOptions = function (e) {
          e = p
            .create(this.loc.clone((e = void 0 === e ? {} : e)), e)
            .resolvedOptions(this);
          return {
            locale: e.locale,
            numberingSystem: e.numberingSystem,
            outputCalendar: e.calendar,
          };
        }),
        (t.toUTC = function (e, t) {
          return (
            void 0 === t && (t = {}),
            this.setZone(c.instance((e = void 0 === e ? 0 : e)), t)
          );
        }),
        (t.toLocal = function () {
          return this.setZone(O.defaultZone);
        }),
        (t.setZone = function (e, t) {
          var n,
            t = void 0 === t ? {} : t,
            r = t.keepLocalTime,
            r = void 0 !== r && r,
            t = t.keepCalendarTime,
            t = void 0 !== t && t;
          return (e = S(e, O.defaultZone)).equals(this.zone)
            ? this
            : e.isValid
              ? ((n = this.ts),
                (r || t) &&
                  ((r = e.offset(this.ts)), (n = nr(this.toObject(), r, e)[0])),
                Z(this, { ts: n, zone: e }))
              : k.invalid(Qn(e));
        }),
        (t.reconfigure = function (e) {
          var e = void 0 === e ? {} : e,
            t = e.locale,
            t = this.loc.clone({
              locale: t,
              numberingSystem: e.numberingSystem,
              outputCalendar: e.outputCalendar,
            });
          return Z(this, { loc: t });
        }),
        (t.setLocale = function (e) {
          return this.reconfigure({ locale: e });
        }),
        (t.set = function (e) {
          if (!this.isValid) return this;
          var t,
            e = Tt(e, yr),
            n = it(e, this.loc),
            r = n.minDaysInFirstWeek,
            n = n.startOfWeek,
            i = !N(e.weekYear) || !N(e.weekNumber) || !N(e.weekday),
            o = !N(e.ordinal),
            a = !N(e.year),
            s = !N(e.month) || !N(e.day),
            u = e.weekYear || e.weekNumber;
          if ((a || s || o) && u)
            throw new w(
              "Can't mix weekYear/weekNumber units with year/month/day or ordinals",
            );
          if (s && o) throw new w("Can't mix ordinal dates with month/day");
          i
            ? (t = tt(l({}, et(this.c, r, n), e), r, n))
            : N(e.ordinal)
              ? ((t = l({}, this.toObject(), e)),
                N(e.day) && (t.day = Math.min(vt(t.year, t.month), t.day)))
              : (t = rt(l({}, nt(this.c), e)));
          a = nr(t, this.o, this.zone);
          return Z(this, { ts: a[0], o: a[1] });
        }),
        (t.plus = function (e) {
          return this.isValid ? Z(this, rr(this, x.fromDurationLike(e))) : this;
        }),
        (t.minus = function (e) {
          return this.isValid
            ? Z(this, rr(this, x.fromDurationLike(e).negate()))
            : this;
        }),
        (t.startOf = function (e, t) {
          ((t = (void 0 === t ? {} : t).useLocaleWeeks),
            (t = void 0 !== t && t));
          if (!this.isValid) return this;
          var n = {},
            e = x.normalizeUnit(e);
          switch (e) {
            case "years":
              n.month = 1;
            case "quarters":
            case "months":
              n.day = 1;
            case "weeks":
            case "days":
              n.hour = 0;
            case "hours":
              n.minute = 0;
            case "minutes":
              n.second = 0;
            case "seconds":
              n.millisecond = 0;
          }
          return (
            "weeks" === e &&
              (t
                ? ((t = this.loc.getStartOfWeek()),
                  this.weekday < t && (n.weekNumber = this.weekNumber - 1),
                  (n.weekday = t))
                : (n.weekday = 1)),
            "quarters" === e &&
              ((t = Math.ceil(this.month / 3)), (n.month = 3 * (t - 1) + 1)),
            this.set(n)
          );
        }),
        (t.endOf = function (e, t) {
          var n;
          return this.isValid
            ? this.plus((((n = {})[e] = 1), n))
                .startOf(e, t)
                .minus(1)
            : this;
        }),
        (t.toFormat = function (e, t) {
          return (
            void 0 === t && (t = {}),
            this.isValid
              ? p
                  .create(this.loc.redefaultToEN(t))
                  .formatDateTimeFromString(this, e)
              : Bn
          );
        }),
        (t.toLocaleString = function (e, t) {
          return (
            void 0 === e && (e = G),
            void 0 === t && (t = {}),
            this.isValid
              ? p.create(this.loc.clone(t), e).formatDateTime(this)
              : Bn
          );
        }),
        (t.toLocaleParts = function (e) {
          return (
            void 0 === e && (e = {}),
            this.isValid
              ? p.create(this.loc.clone(e), e).formatDateTimeParts(this)
              : []
          );
        }),
        (t.toISO = function (e) {
          var t,
            e = void 0 === e ? {} : e,
            n = e.format,
            r = e.suppressSeconds,
            r = void 0 !== r && r,
            i = e.suppressMilliseconds,
            i = void 0 !== i && i,
            o = e.includeOffset,
            o = void 0 === o || o,
            e = e.extendedZone,
            e = void 0 !== e && e;
          return this.isValid
            ? ((t = ar(
                this,
                (n = "extended" === (void 0 === n ? "extended" : n)),
              )),
              (t += "T") + sr(this, n, r, i, o, e))
            : null;
        }),
        (t.toISODate = function (e) {
          e = (void 0 === e ? {} : e).format;
          return this.isValid
            ? ar(this, "extended" === (void 0 === e ? "extended" : e))
            : null;
        }),
        (t.toISOWeekDate = function () {
          return or(this, "kkkk-'W'WW-c");
        }),
        (t.toISOTime = function (e) {
          var e = void 0 === e ? {} : e,
            t = e.suppressMilliseconds,
            n = e.suppressSeconds,
            r = e.includeOffset,
            i = e.includePrefix,
            o = e.extendedZone,
            e = e.format;
          return this.isValid
            ? (void 0 !== i && i ? "T" : "") +
                sr(
                  this,
                  "extended" === (void 0 === e ? "extended" : e),
                  void 0 !== n && n,
                  void 0 !== t && t,
                  void 0 === r || r,
                  void 0 !== o && o,
                )
            : null;
        }),
        (t.toRFC2822 = function () {
          return or(this, "EEE, dd LLL yyyy HH:mm:ss ZZZ", !1);
        }),
        (t.toHTTP = function () {
          return or(this.toUTC(), "EEE, dd LLL yyyy HH:mm:ss 'GMT'");
        }),
        (t.toSQLDate = function () {
          return this.isValid ? ar(this, !0) : null;
        }),
        (t.toSQLTime = function (e) {
          var e = void 0 === e ? {} : e,
            t = e.includeOffset,
            t = void 0 === t || t,
            n = e.includeZone,
            n = void 0 !== n && n,
            e = e.includeOffsetSpace,
            r = "HH:mm:ss.SSS";
          return (
            (n || t) &&
              ((void 0 === e || e) && (r += " "),
              n ? (r += "z") : t && (r += "ZZ")),
            or(this, r, !0)
          );
        }),
        (t.toSQL = function (e) {
          return (
            void 0 === e && (e = {}),
            this.isValid ? this.toSQLDate() + " " + this.toSQLTime(e) : null
          );
        }),
        (t.toString = function () {
          return this.isValid ? this.toISO() : Bn;
        }),
        (t[e] = function () {
          return this.isValid
            ? "DateTime { ts: " +
                this.toISO() +
                ", zone: " +
                this.zone.name +
                ", locale: " +
                this.locale +
                " }"
            : "DateTime { Invalid, reason: " + this.invalidReason + " }";
        }),
        (t.valueOf = function () {
          return this.toMillis();
        }),
        (t.toMillis = function () {
          return this.isValid ? this.ts : NaN;
        }),
        (t.toSeconds = function () {
          return this.isValid ? this.ts / 1e3 : NaN;
        }),
        (t.toUnixInteger = function () {
          return this.isValid ? Math.floor(this.ts / 1e3) : NaN;
        }),
        (t.toJSON = function () {
          return this.toISO();
        }),
        (t.toBSON = function () {
          return this.toJSDate();
        }),
        (t.toObject = function (e) {
          var t;
          return (
            void 0 === e && (e = {}),
            this.isValid
              ? ((t = l({}, this.c)),
                e.includeConfig &&
                  ((t.outputCalendar = this.outputCalendar),
                  (t.numberingSystem = this.loc.numberingSystem),
                  (t.locale = this.loc.locale)),
                t)
              : {}
          );
        }),
        (t.toJSDate = function () {
          return new Date(this.isValid ? this.ts : NaN);
        }),
        (t.diff = function (e, t, n) {
          var r;
          return (
            void 0 === t && (t = "milliseconds"),
            void 0 === n && (n = {}),
            this.isValid && e.isValid
              ? ((n = l(
                  {
                    locale: this.locale,
                    numberingSystem: this.numberingSystem,
                  },
                  n,
                )),
                (t = t),
                (t = (Array.isArray(t) ? t : [t]).map(x.normalizeUnit)),
                (e = Zn(
                  (r = e.valueOf() > this.valueOf()) ? this : e,
                  r ? e : this,
                  t,
                  n,
                )),
                r ? e.negate() : e)
              : x.invalid("created by diffing an invalid DateTime")
          );
        }),
        (t.diffNow = function (e, t) {
          return (
            void 0 === e && (e = "milliseconds"),
            void 0 === t && (t = {}),
            this.diff(k.now(), e, t)
          );
        }),
        (t.until = function (e) {
          return this.isValid ? xn.fromDateTimes(this, e) : this;
        }),
        (t.hasSame = function (e, t, n) {
          var r;
          return (
            !!this.isValid &&
            ((r = e.valueOf()),
            (e = this.setZone(e.zone, { keepLocalTime: !0 })).startOf(t, n) <=
              r) &&
            r <= e.endOf(t, n)
          );
        }),
        (t.equals = function (e) {
          return (
            this.isValid &&
            e.isValid &&
            this.valueOf() === e.valueOf() &&
            this.zone.equals(e.zone) &&
            this.loc.equals(e.loc)
          );
        }),
        (t.toRelative = function (e) {
          var t, n, r, i;
          return this.isValid
            ? ((t =
                (e = void 0 === e ? {} : e).base ||
                k.fromObject({}, { zone: this.zone })),
              (n = e.padding ? (this < t ? -e.padding : e.padding) : 0),
              (r = ["years", "months", "days", "hours", "minutes", "seconds"]),
              (i = e.unit),
              Array.isArray(e.unit) && ((r = e.unit), (i = void 0)),
              gr(
                t,
                this.plus(n),
                l({}, e, { numeric: "always", units: r, unit: i }),
              ))
            : null;
        }),
        (t.toRelativeCalendar = function (e) {
          return (
            void 0 === e && (e = {}),
            this.isValid
              ? gr(
                  e.base || k.fromObject({}, { zone: this.zone }),
                  this,
                  l({}, e, {
                    numeric: "auto",
                    units: ["years", "months", "days"],
                    calendary: !0,
                  }),
                )
              : null
          );
        }),
        (k.min = function () {
          for (var e = arguments.length, t = new Array(e), n = 0; n < e; n++)
            t[n] = arguments[n];
          if (t.every(k.isDateTime))
            return ct(
              t,
              function (e) {
                return e.valueOf();
              },
              Math.min,
            );
          throw new u("min requires all arguments be DateTimes");
        }),
        (k.max = function () {
          for (var e = arguments.length, t = new Array(e), n = 0; n < e; n++)
            t[n] = arguments[n];
          if (t.every(k.isDateTime))
            return ct(
              t,
              function (e) {
                return e.valueOf();
              },
              Math.max,
            );
          throw new u("max requires all arguments be DateTimes");
        }),
        (k.fromFormatExplain = function (e, t, n) {
          var n = (n = void 0 === n ? {} : n),
            r = n.locale,
            n = n.numberingSystem;
          return Gn(
            b.fromOpts({
              locale: void 0 === r ? null : r,
              numberingSystem: void 0 === n ? null : n,
              defaultToEN: !0,
            }),
            e,
            t,
          );
        }),
        (k.fromStringExplain = function (e, t, n) {
          return k.fromFormatExplain(e, t, (n = void 0 === n ? {} : n));
        }),
        (k.buildFormatParser = function (e, t) {
          var t = (t = void 0 === t ? {} : t),
            n = t.locale,
            t = t.numberingSystem,
            n = b.fromOpts({
              locale: void 0 === n ? null : n,
              numberingSystem: void 0 === t ? null : t,
              defaultToEN: !0,
            });
          return new Jn(n, e);
        }),
        (k.fromFormatParser = function (e, t, n) {
          if ((void 0 === n && (n = {}), N(e) || N(t)))
            throw new u(
              "fromFormatParser requires an input string and a format parser",
            );
          var r,
            i,
            o,
            a = n,
            s = a.locale,
            a = a.numberingSystem,
            s = b.fromOpts({
              locale: void 0 === s ? null : s,
              numberingSystem: void 0 === a ? null : a,
              defaultToEN: !0,
            });
          if (s.equals(t.locale))
            return (
              (r = (a = t.explainFromTokens(e)).result),
              (i = a.zone),
              (o = a.specificOffset),
              (a = a.invalidReason)
                ? k.invalid(a)
                : ir(r, i, n, "format " + t.format, e, o)
            );
          throw new u(
            "fromFormatParser called with a locale of " +
              s +
              ", but the format parser was created for " +
              t.locale,
          );
        }),
        i(
          k,
          [
            {
              key: "isValid",
              get: function () {
                return null === this.invalid;
              },
            },
            {
              key: "invalidReason",
              get: function () {
                return this.invalid ? this.invalid.reason : null;
              },
            },
            {
              key: "invalidExplanation",
              get: function () {
                return this.invalid ? this.invalid.explanation : null;
              },
            },
            {
              key: "locale",
              get: function () {
                return this.isValid ? this.loc.locale : null;
              },
            },
            {
              key: "numberingSystem",
              get: function () {
                return this.isValid ? this.loc.numberingSystem : null;
              },
            },
            {
              key: "outputCalendar",
              get: function () {
                return this.isValid ? this.loc.outputCalendar : null;
              },
            },
            {
              key: "zone",
              get: function () {
                return this._zone;
              },
            },
            {
              key: "zoneName",
              get: function () {
                return this.isValid ? this.zone.name : null;
              },
            },
            {
              key: "year",
              get: function () {
                return this.isValid ? this.c.year : NaN;
              },
            },
            {
              key: "quarter",
              get: function () {
                return this.isValid ? Math.ceil(this.c.month / 3) : NaN;
              },
            },
            {
              key: "month",
              get: function () {
                return this.isValid ? this.c.month : NaN;
              },
            },
            {
              key: "day",
              get: function () {
                return this.isValid ? this.c.day : NaN;
              },
            },
            {
              key: "hour",
              get: function () {
                return this.isValid ? this.c.hour : NaN;
              },
            },
            {
              key: "minute",
              get: function () {
                return this.isValid ? this.c.minute : NaN;
              },
            },
            {
              key: "second",
              get: function () {
                return this.isValid ? this.c.second : NaN;
              },
            },
            {
              key: "millisecond",
              get: function () {
                return this.isValid ? this.c.millisecond : NaN;
              },
            },
            {
              key: "weekYear",
              get: function () {
                return this.isValid ? Kn(this).weekYear : NaN;
              },
            },
            {
              key: "weekNumber",
              get: function () {
                return this.isValid ? Kn(this).weekNumber : NaN;
              },
            },
            {
              key: "weekday",
              get: function () {
                return this.isValid ? Kn(this).weekday : NaN;
              },
            },
            {
              key: "isWeekend",
              get: function () {
                return (
                  this.isValid &&
                  this.loc.getWeekendDays().includes(this.weekday)
                );
              },
            },
            {
              key: "localWeekday",
              get: function () {
                return this.isValid ? Xn(this).weekday : NaN;
              },
            },
            {
              key: "localWeekNumber",
              get: function () {
                return this.isValid ? Xn(this).weekNumber : NaN;
              },
            },
            {
              key: "localWeekYear",
              get: function () {
                return this.isValid ? Xn(this).weekYear : NaN;
              },
            },
            {
              key: "ordinal",
              get: function () {
                return this.isValid ? nt(this.c).ordinal : NaN;
              },
            },
            {
              key: "monthShort",
              get: function () {
                return this.isValid
                  ? Fn.months("short", { locObj: this.loc })[this.month - 1]
                  : null;
              },
            },
            {
              key: "monthLong",
              get: function () {
                return this.isValid
                  ? Fn.months("long", { locObj: this.loc })[this.month - 1]
                  : null;
              },
            },
            {
              key: "weekdayShort",
              get: function () {
                return this.isValid
                  ? Fn.weekdays("short", { locObj: this.loc })[this.weekday - 1]
                  : null;
              },
            },
            {
              key: "weekdayLong",
              get: function () {
                return this.isValid
                  ? Fn.weekdays("long", { locObj: this.loc })[this.weekday - 1]
                  : null;
              },
            },
            {
              key: "offset",
              get: function () {
                return this.isValid ? +this.o : NaN;
              },
            },
            {
              key: "offsetNameShort",
              get: function () {
                return this.isValid
                  ? this.zone.offsetName(this.ts, {
                      format: "short",
                      locale: this.locale,
                    })
                  : null;
              },
            },
            {
              key: "offsetNameLong",
              get: function () {
                return this.isValid
                  ? this.zone.offsetName(this.ts, {
                      format: "long",
                      locale: this.locale,
                    })
                  : null;
              },
            },
            {
              key: "isOffsetFixed",
              get: function () {
                return this.isValid ? this.zone.isUniversal : null;
              },
            },
            {
              key: "isInDST",
              get: function () {
                return (
                  !this.isOffsetFixed &&
                  (this.offset > this.set({ month: 1, day: 1 }).offset ||
                    this.offset > this.set({ month: 5 }).offset)
                );
              },
            },
            {
              key: "isInLeapYear",
              get: function () {
                return mt(this.year);
              },
            },
            {
              key: "daysInMonth",
              get: function () {
                return vt(this.year, this.month);
              },
            },
            {
              key: "daysInYear",
              get: function () {
                return this.isValid ? yt(this.year) : NaN;
              },
            },
            {
              key: "weeksInWeekYear",
              get: function () {
                return this.isValid ? kt(this.weekYear) : NaN;
              },
            },
            {
              key: "weeksInLocalWeekYear",
              get: function () {
                return this.isValid
                  ? kt(
                      this.localWeekYear,
                      this.loc.getMinDaysInFirstWeek(),
                      this.loc.getStartOfWeek(),
                    )
                  : NaN;
              },
            },
          ],
          [
            {
              key: "DATE_SHORT",
              get: function () {
                return G;
              },
            },
            {
              key: "DATE_MED",
              get: function () {
                return $;
              },
            },
            {
              key: "DATE_MED_WITH_WEEKDAY",
              get: function () {
                return B;
              },
            },
            {
              key: "DATE_FULL",
              get: function () {
                return Q;
              },
            },
            {
              key: "DATE_HUGE",
              get: function () {
                return K;
              },
            },
            {
              key: "TIME_SIMPLE",
              get: function () {
                return X;
              },
            },
            {
              key: "TIME_WITH_SECONDS",
              get: function () {
                return ee;
              },
            },
            {
              key: "TIME_WITH_SHORT_OFFSET",
              get: function () {
                return te;
              },
            },
            {
              key: "TIME_WITH_LONG_OFFSET",
              get: function () {
                return ne;
              },
            },
            {
              key: "TIME_24_SIMPLE",
              get: function () {
                return re;
              },
            },
            {
              key: "TIME_24_WITH_SECONDS",
              get: function () {
                return ie;
              },
            },
            {
              key: "TIME_24_WITH_SHORT_OFFSET",
              get: function () {
                return oe;
              },
            },
            {
              key: "TIME_24_WITH_LONG_OFFSET",
              get: function () {
                return ae;
              },
            },
            {
              key: "DATETIME_SHORT",
              get: function () {
                return se;
              },
            },
            {
              key: "DATETIME_SHORT_WITH_SECONDS",
              get: function () {
                return ue;
              },
            },
            {
              key: "DATETIME_MED",
              get: function () {
                return le;
              },
            },
            {
              key: "DATETIME_MED_WITH_SECONDS",
              get: function () {
                return ce;
              },
            },
            {
              key: "DATETIME_MED_WITH_WEEKDAY",
              get: function () {
                return fe;
              },
            },
            {
              key: "DATETIME_FULL",
              get: function () {
                return de;
              },
            },
            {
              key: "DATETIME_FULL_WITH_SECONDS",
              get: function () {
                return he;
              },
            },
            {
              key: "DATETIME_HUGE",
              get: function () {
                return me;
              },
            },
            {
              key: "DATETIME_HUGE_WITH_SECONDS",
              get: function () {
                return ye;
              },
            },
          ],
        ),
        k
      );
    })(Symbol.for("nodejs.util.inspect.custom"));
  function wr(e) {
    if (W.isDateTime(e)) return e;
    if (e && e.valueOf && v(e.valueOf())) return W.fromJSDate(e);
    if (e && "object" == typeof e) return W.fromObject(e);
    throw new u("Unknown datetime argument: " + e + ", of type " + typeof e);
  }
  return (
    (e.DateTime = W),
    (e.Duration = x),
    (e.FixedOffsetZone = c),
    (e.IANAZone = a),
    (e.Info = Fn),
    (e.Interval = xn),
    (e.InvalidZone = Le),
    (e.Settings = O),
    (e.SystemZone = ge),
    (e.VERSION = "3.5.0"),
    (e.Zone = r),
    Object.defineProperty(e, "__esModule", { value: !0 }),
    e
  );
})({});
((e, t) => {
  "object" == typeof exports && "undefined" != typeof module
    ? (module.exports = t())
    : "function" == typeof define && define.amd
      ? define(t)
      : (((e =
          "undefined" != typeof globalThis ? globalThis : e || self).Vimeo =
          e.Vimeo || {}),
        (e.Vimeo.Player = t()));
})(this, function () {
  function r(t, e) {
    var n,
      r = Object.keys(t);
    return (
      Object.getOwnPropertySymbols &&
        ((n = Object.getOwnPropertySymbols(t)),
        e &&
          (n = n.filter(function (e) {
            return Object.getOwnPropertyDescriptor(t, e).enumerable;
          })),
        r.push.apply(r, n)),
      r
    );
  }
  function c(t) {
    for (var e = 1; e < arguments.length; e++) {
      var n = null != arguments[e] ? arguments[e] : {};
      e % 2
        ? r(Object(n), !0).forEach(function (e) {
            f(t, e, n[e]);
          })
        : Object.getOwnPropertyDescriptors
          ? Object.defineProperties(t, Object.getOwnPropertyDescriptors(n))
          : r(Object(n)).forEach(function (e) {
              Object.defineProperty(
                t,
                e,
                Object.getOwnPropertyDescriptor(n, e),
              );
            });
    }
    return t;
  }
  function P() {
    P = function () {
      return a;
    };
    var a = {},
      e = Object.prototype,
      c = e.hasOwnProperty,
      l =
        Object.defineProperty ||
        function (e, t, n) {
          e[t] = n.value;
        },
      t = "function" == typeof Symbol ? Symbol : {},
      r = t.iterator || "@@iterator",
      n = t.asyncIterator || "@@asyncIterator",
      o = t.toStringTag || "@@toStringTag";
    function i(e, t, n) {
      return (
        Object.defineProperty(e, t, {
          value: n,
          enumerable: !0,
          configurable: !0,
          writable: !0,
        }),
        e[t]
      );
    }
    try {
      i({}, "");
    } catch (e) {
      i = function (e, t, n) {
        return (e[t] = n);
      };
    }
    function u(e, t, n, r) {
      var o,
        i,
        a,
        u,
        t = t && t.prototype instanceof d ? t : d,
        t = Object.create(t.prototype),
        r = new k(r || []);
      return (
        l(t, "_invoke", {
          value:
            ((o = e),
            (i = n),
            (a = r),
            (u = "suspendedStart"),
            function (e, t) {
              if ("executing" === u)
                throw new Error("Generator is already running");
              if ("completed" === u) {
                if ("throw" === e) throw t;
                return x();
              }
              for (a.method = e, a.arg = t; ; ) {
                var n = a.delegate;
                if (n) {
                  n = (function e(t, n) {
                    var r = n.method,
                      o = t.iterator[r];
                    return void 0 === o
                      ? ((n.delegate = null),
                        ("throw" === r &&
                          t.iterator.return &&
                          ((n.method = "return"),
                          (n.arg = void 0),
                          e(t, n),
                          "throw" === n.method)) ||
                          ("return" !== r &&
                            ((n.method = "throw"),
                            (n.arg = new TypeError(
                              "The iterator does not provide a '" +
                                r +
                                "' method",
                            )))),
                        f)
                      : "throw" === (r = s(o, t.iterator, n.arg)).type
                        ? ((n.method = "throw"),
                          (n.arg = r.arg),
                          (n.delegate = null),
                          f)
                        : (o = r.arg)
                          ? o.done
                            ? ((n[t.resultName] = o.value),
                              (n.next = t.nextLoc),
                              "return" !== n.method &&
                                ((n.method = "next"), (n.arg = void 0)),
                              (n.delegate = null),
                              f)
                            : o
                          : ((n.method = "throw"),
                            (n.arg = new TypeError(
                              "iterator result is not an object",
                            )),
                            (n.delegate = null),
                            f);
                  })(n, a);
                  if (n) {
                    if (n === f) continue;
                    return n;
                  }
                }
                if ("next" === a.method) a.sent = a._sent = a.arg;
                else if ("throw" === a.method) {
                  if ("suspendedStart" === u) throw ((u = "completed"), a.arg);
                  a.dispatchException(a.arg);
                } else "return" === a.method && a.abrupt("return", a.arg);
                u = "executing";
                n = s(o, i, a);
                if ("normal" === n.type) {
                  if (
                    ((u = a.done ? "completed" : "suspendedYield"), n.arg === f)
                  )
                    continue;
                  return { value: n.arg, done: a.done };
                }
                "throw" === n.type &&
                  ((u = "completed"), (a.method = "throw"), (a.arg = n.arg));
              }
            }),
        }),
        t
      );
    }
    function s(e, t, n) {
      try {
        return { type: "normal", arg: e.call(t, n) };
      } catch (e) {
        return { type: "throw", arg: e };
      }
    }
    a.wrap = u;
    var f = {};
    function d() {}
    function h() {}
    function p() {}
    var t = {},
      y =
        (i(t, r, function () {
          return this;
        }),
        Object.getPrototypeOf),
      y = y && y(y(E([]))),
      v =
        (y && y !== e && c.call(y, r) && (t = y),
        (p.prototype = d.prototype = Object.create(t)));
    function m(e) {
      ["next", "throw", "return"].forEach(function (t) {
        i(e, t, function (e) {
          return this._invoke(t, e);
        });
      });
    }
    function g(a, u) {
      var t;
      l(this, "_invoke", {
        value: function (n, r) {
          function e() {
            return new u(function (e, t) {
              !(function t(e, n, r, o) {
                var i,
                  e = s(a[e], a, n);
                return "throw" !== e.type
                  ? (n = (i = e.arg).value) &&
                    "object" == typeof n &&
                    c.call(n, "__await")
                    ? u.resolve(n.__await).then(
                        function (e) {
                          t("next", e, r, o);
                        },
                        function (e) {
                          t("throw", e, r, o);
                        },
                      )
                    : u.resolve(n).then(
                        function (e) {
                          ((i.value = e), r(i));
                        },
                        function (e) {
                          return t("throw", e, r, o);
                        },
                      )
                  : void o(e.arg);
              })(n, r, e, t);
            });
          }
          return (t = t ? t.then(e, e) : e());
        },
      });
    }
    function w(e) {
      var t = { tryLoc: e[0] };
      (1 in e && (t.catchLoc = e[1]),
        2 in e && ((t.finallyLoc = e[2]), (t.afterLoc = e[3])),
        this.tryEntries.push(t));
    }
    function b(e) {
      var t = e.completion || {};
      ((t.type = "normal"), delete t.arg, (e.completion = t));
    }
    function k(e) {
      ((this.tryEntries = [{ tryLoc: "root" }]),
        e.forEach(w, this),
        this.reset(!0));
    }
    function E(t) {
      if (t) {
        var n,
          e = t[r];
        if (e) return e.call(t);
        if ("function" == typeof t.next) return t;
        if (!isNaN(t.length))
          return (
            (n = -1),
            ((e = function e() {
              for (; ++n < t.length; )
                if (c.call(t, n)) return ((e.value = t[n]), (e.done = !1), e);
              return ((e.value = void 0), (e.done = !0), e);
            }).next = e)
          );
      }
      return { next: x };
    }
    function x() {
      return { value: void 0, done: !0 };
    }
    return (
      l(v, "constructor", { value: (h.prototype = p), configurable: !0 }),
      l(p, "constructor", { value: h, configurable: !0 }),
      (h.displayName = i(p, o, "GeneratorFunction")),
      (a.isGeneratorFunction = function (e) {
        e = "function" == typeof e && e.constructor;
        return (
          !!e && (e === h || "GeneratorFunction" === (e.displayName || e.name))
        );
      }),
      (a.mark = function (e) {
        return (
          Object.setPrototypeOf
            ? Object.setPrototypeOf(e, p)
            : ((e.__proto__ = p), i(e, o, "GeneratorFunction")),
          (e.prototype = Object.create(v)),
          e
        );
      }),
      (a.awrap = function (e) {
        return { __await: e };
      }),
      m(g.prototype),
      i(g.prototype, n, function () {
        return this;
      }),
      (a.AsyncIterator = g),
      (a.async = function (e, t, n, r, o) {
        void 0 === o && (o = Promise);
        var i = new g(u(e, t, n, r), o);
        return a.isGeneratorFunction(t)
          ? i
          : i.next().then(function (e) {
              return e.done ? e.value : i.next();
            });
      }),
      m(v),
      i(v, o, "Generator"),
      i(v, r, function () {
        return this;
      }),
      i(v, "toString", function () {
        return "[object Generator]";
      }),
      (a.keys = function (e) {
        var t,
          n = Object(e),
          r = [];
        for (t in n) r.push(t);
        return (
          r.reverse(),
          function e() {
            for (; r.length; ) {
              var t = r.pop();
              if (t in n) return ((e.value = t), (e.done = !1), e);
            }
            return ((e.done = !0), e);
          }
        );
      }),
      (a.values = E),
      (k.prototype = {
        constructor: k,
        reset: function (e) {
          if (
            ((this.prev = 0),
            (this.next = 0),
            (this.sent = this._sent = void 0),
            (this.done = !1),
            (this.delegate = null),
            (this.method = "next"),
            (this.arg = void 0),
            this.tryEntries.forEach(b),
            !e)
          )
            for (var t in this)
              "t" === t.charAt(0) &&
                c.call(this, t) &&
                !isNaN(+t.slice(1)) &&
                (this[t] = void 0);
        },
        stop: function () {
          this.done = !0;
          var e = this.tryEntries[0].completion;
          if ("throw" === e.type) throw e.arg;
          return this.rval;
        },
        dispatchException: function (n) {
          if (this.done) throw n;
          var r = this;
          function e(e, t) {
            return (
              (i.type = "throw"),
              (i.arg = n),
              (r.next = e),
              t && ((r.method = "next"), (r.arg = void 0)),
              !!t
            );
          }
          for (var t = this.tryEntries.length - 1; 0 <= t; --t) {
            var o = this.tryEntries[t],
              i = o.completion;
            if ("root" === o.tryLoc) return e("end");
            if (o.tryLoc <= this.prev) {
              var a = c.call(o, "catchLoc"),
                u = c.call(o, "finallyLoc");
              if (a && u) {
                if (this.prev < o.catchLoc) return e(o.catchLoc, !0);
                if (this.prev < o.finallyLoc) return e(o.finallyLoc);
              } else if (a) {
                if (this.prev < o.catchLoc) return e(o.catchLoc, !0);
              } else {
                if (!u)
                  throw new Error("try statement without catch or finally");
                if (this.prev < o.finallyLoc) return e(o.finallyLoc);
              }
            }
          }
        },
        abrupt: function (e, t) {
          for (var n = this.tryEntries.length - 1; 0 <= n; --n) {
            var r = this.tryEntries[n];
            if (
              r.tryLoc <= this.prev &&
              c.call(r, "finallyLoc") &&
              this.prev < r.finallyLoc
            ) {
              var o = r;
              break;
            }
          }
          var i = (o =
            o &&
            ("break" === e || "continue" === e) &&
            o.tryLoc <= t &&
            t <= o.finallyLoc
              ? null
              : o)
            ? o.completion
            : {};
          return (
            (i.type = e),
            (i.arg = t),
            o
              ? ((this.method = "next"), (this.next = o.finallyLoc), f)
              : this.complete(i)
          );
        },
        complete: function (e, t) {
          if ("throw" === e.type) throw e.arg;
          return (
            "break" === e.type || "continue" === e.type
              ? (this.next = e.arg)
              : "return" === e.type
                ? ((this.rval = this.arg = e.arg),
                  (this.method = "return"),
                  (this.next = "end"))
                : "normal" === e.type && t && (this.next = t),
            f
          );
        },
        finish: function (e) {
          for (var t = this.tryEntries.length - 1; 0 <= t; --t) {
            var n = this.tryEntries[t];
            if (n.finallyLoc === e)
              return (this.complete(n.completion, n.afterLoc), b(n), f);
          }
        },
        catch: function (e) {
          for (var t = this.tryEntries.length - 1; 0 <= t; --t) {
            var n,
              r,
              o = this.tryEntries[t];
            if (o.tryLoc === e)
              return (
                "throw" === (r = o.completion).type && ((n = r.arg), b(o)),
                n
              );
          }
          throw new Error("illegal catch attempt");
        },
        delegateYield: function (e, t, n) {
          return (
            (this.delegate = { iterator: E(e), resultName: t, nextLoc: n }),
            "next" === this.method && (this.arg = void 0),
            f
          );
        },
      }),
      a
    );
  }
  function l(e, t, n, r, o, i, a) {
    try {
      var u = e[i](a),
        c = u.value;
    } catch (e) {
      return n(e);
    }
    u.done ? t(c) : Promise.resolve(c).then(r, o);
  }
  function d(u) {
    return function () {
      var e = this,
        a = arguments;
      return new Promise(function (t, n) {
        var r = u.apply(e, a);
        function o(e) {
          l(r, t, n, o, i, "next", e);
        }
        function i(e) {
          l(r, t, n, o, i, "throw", e);
        }
        o(void 0);
      });
    };
  }
  function s(e, t) {
    if (!(e instanceof t))
      throw new TypeError("Cannot call a class as a function");
  }
  function o(e, t) {
    for (var n = 0; n < t.length; n++) {
      var r = t[n];
      ((r.enumerable = r.enumerable || !1),
        (r.configurable = !0),
        "value" in r && (r.writable = !0),
        Object.defineProperty(e, W(r.key), r));
    }
  }
  function R(e, t, n) {
    (t && o(e.prototype, t),
      n && o(e, n),
      Object.defineProperty(e, "prototype", { writable: !1 }));
  }
  function f(e, t, n) {
    (t = W(t)) in e
      ? Object.defineProperty(e, t, {
          value: n,
          enumerable: !0,
          configurable: !0,
          writable: !0,
        })
      : (e[t] = n);
  }
  function i(e) {
    return (i = Object.setPrototypeOf
      ? Object.getPrototypeOf.bind()
      : function (e) {
          return e.__proto__ || Object.getPrototypeOf(e);
        })(e);
  }
  function h(e, t) {
    return (h = Object.setPrototypeOf
      ? Object.setPrototypeOf.bind()
      : function (e, t) {
          return ((e.__proto__ = t), e);
        })(e, t);
  }
  function q() {
    if ("undefined" == typeof Reflect || !Reflect.construct) return !1;
    if (Reflect.construct.sham) return !1;
    if ("function" == typeof Proxy) return !0;
    try {
      return (
        Boolean.prototype.valueOf.call(
          Reflect.construct(Boolean, [], function () {}),
        ),
        !0
      );
    } catch (e) {
      return !1;
    }
  }
  function I(e, t, n) {
    return (I = q()
      ? Reflect.construct.bind()
      : function (e, t, n) {
          var r = [null],
            t = (r.push.apply(r, t), new (Function.bind.apply(e, r))());
          return (n && h(t, n.prototype), t);
        }).apply(null, arguments);
  }
  function V(e) {
    var n = "function" == typeof Map ? new Map() : void 0;
    return (function (e) {
      if (
        null === e ||
        -1 === Function.toString.call(e).indexOf("[native code]")
      )
        return e;
      if ("function" != typeof e)
        throw new TypeError(
          "Super expression must either be null or a function",
        );
      if (void 0 !== n) {
        if (n.has(e)) return n.get(e);
        n.set(e, t);
      }
      function t() {
        return I(e, arguments, i(this).constructor);
      }
      return (
        (t.prototype = Object.create(e.prototype, {
          constructor: {
            value: t,
            enumerable: !1,
            writable: !0,
            configurable: !0,
          },
        })),
        h(t, e)
      );
    })(e);
  }
  function p(e) {
    if (void 0 === e)
      throw new ReferenceError(
        "this hasn't been initialised - super() hasn't been called",
      );
    return e;
  }
  function D(r) {
    var o = q();
    return function () {
      var e = i(r),
        t = this,
        n = o
          ? ((n = i(this).constructor), Reflect.construct(e, arguments, n))
          : e.apply(this, arguments);
      if (n && ("object" == typeof n || "function" == typeof n)) return n;
      if (void 0 !== n)
        throw new TypeError(
          "Derived constructors may only return object or undefined",
        );
      return p(t);
    };
  }
  function W(e) {
    e = ((e) => {
      if ("object" != typeof e || null === e) return e;
      var t = e[Symbol.toPrimitive];
      if (void 0 === t) return String(e);
      if ("object" != typeof (t = t.call(e, "string"))) return t;
      throw new TypeError("@@toPrimitive must return a primitive value.");
    })(e);
    return "symbol" == typeof e ? e : String(e);
  }
  var z =
    "undefined" != typeof global &&
    "[object global]" === {}.toString.call(global);
  function U(e, t) {
    return 0 === e.indexOf(t.toLowerCase())
      ? e
      : ""
          .concat(t.toLowerCase())
          .concat(e.substr(0, 1).toUpperCase())
          .concat(e.substr(1));
  }
  function y(e) {
    return /^(https?:)?\/\/((((player|www)\.)?vimeo\.com)|((player\.)?[a-zA-Z0-9-]+\.(videoji\.(hk|cn)|vimeo\.work)))(?=$|\/)/.test(
      e,
    );
  }
  function G(e) {
    return /^https:\/\/player\.((vimeo\.com)|([a-zA-Z0-9-]+\.(videoji\.(hk|cn)|vimeo\.work)))\/video\/\d+/.test(
      e,
    );
  }
  function B(e) {
    var t,
      e = 0 < arguments.length && void 0 !== e ? e : {},
      n = e.id,
      e = n || e.url;
    if (!e)
      throw new Error(
        "An id or url must be passed, either in an options object or as a data-vimeo-id or data-vimeo-url attribute.",
      );
    if (((t = e), !isNaN(parseFloat(t)) && isFinite(t) && Math.floor(t) == t))
      return "https://vimeo.com/".concat(e);
    if (y(e)) return e.replace("http:", "https:");
    if (n) throw new TypeError("“".concat(n, "” is not a valid video id."));
    throw new TypeError("“".concat(e, "” is not a vimeo.com url."));
  }
  function H(t, e, n, r, o) {
    var i = 3 < arguments.length && void 0 !== r ? r : "addEventListener",
      a = 4 < arguments.length && void 0 !== o ? o : "removeEventListener",
      u = "string" == typeof e ? [e] : e;
    return (
      u.forEach(function (e) {
        t[i](e, n);
      }),
      {
        cancel: function () {
          return u.forEach(function (e) {
            return t[a](e, n);
          });
        },
      }
    );
  }
  var e = void 0 !== Array.prototype.indexOf,
    t = "undefined" != typeof window && void 0 !== window.postMessage;
  if (!(z || (e && t)))
    throw new Error(
      "Sorry, the Vimeo Player API is not available in this browser.",
    );
  var Y,
    Q,
    a,
    e =
      "undefined" != typeof globalThis
        ? globalThis
        : "undefined" != typeof window
          ? window
          : "undefined" != typeof global
            ? global
            : "undefined" != typeof self
              ? self
              : {};
  function n() {
    if (void 0 === this)
      throw new TypeError("Constructor WeakMap requires 'new'");
    if ((a(this, "_id", "_WeakMap_" + J() + "." + J()), 0 < arguments.length))
      throw new TypeError("WeakMap iterable is not supported");
  }
  function u(e, t) {
    if (!v(e) || !Y.call(e, "_id"))
      throw new TypeError(
        t + " method called on incompatible receiver " + typeof e,
      );
  }
  function J() {
    return Math.random().toString().substring(2);
  }
  function v(e) {
    return Object(e) === e;
  }
  (t =
    "undefined" != typeof globalThis
      ? globalThis
      : "undefined" != typeof self
        ? self
        : "undefined" != typeof window
          ? window
          : e).WeakMap ||
    ((Y = Object.prototype.hasOwnProperty),
    (Q =
      Object.defineProperty &&
      (() => {
        try {
          return 1 === Object.defineProperty({}, "x", { value: 1 }).x;
        } catch (e) {}
      })()),
    (t.WeakMap =
      ((a = function (e, t, n) {
        Q
          ? Object.defineProperty(e, t, {
              configurable: !0,
              writable: !0,
              value: n,
            })
          : (e[t] = n);
      })(n.prototype, "delete", function (e) {
        var t;
        return (
          u(this, "delete"),
          !!v(e) &&
            !(!(t = e[this._id]) || t[0] !== e || (delete e[this._id], 0))
        );
      }),
      a(n.prototype, "get", function (e) {
        var t;
        return (
          u(this, "get"),
          v(e) && (t = e[this._id]) && t[0] === e ? t[1] : void 0
        );
      }),
      a(n.prototype, "has", function (e) {
        var t;
        return (u(this, "has"), !!v(e) && !(!(t = e[this._id]) || t[0] !== e));
      }),
      a(n.prototype, "set", function (e, t) {
        var n;
        if ((u(this, "set"), v(e)))
          return (
            (n = e[this._id]) && n[0] === e
              ? (n[1] = t)
              : a(e, this._id, [e, t]),
            this
          );
        throw new TypeError("Invalid value used as weak map key");
      }),
      a(n, "_polyfill", !0),
      n)));
  ((m = t = { exports: {} }),
    ((e = e)[(g = "Promise")] =
      e[g] ||
      (function () {
        var e,
          n,
          r,
          o,
          i,
          a,
          u = Object.prototype.toString,
          c =
            "undefined" != typeof setImmediate
              ? function (e) {
                  return setImmediate(e);
                }
              : setTimeout;
        try {
          (Object.defineProperty({}, "x", {}),
            (e = function (e, t, n, r) {
              return Object.defineProperty(e, t, {
                value: n,
                writable: !0,
                configurable: !1 !== r,
              });
            }));
        } catch (u) {
          e = function (e, t, n) {
            return ((e[t] = n), e);
          };
        }
        function l(e, t) {
          ((this.fn = e), (this.self = t), (this.next = void 0));
        }
        function s(e, t) {
          (r.add(e, t), (n = n || c(r.drain)));
        }
        function f(e) {
          var t,
            n = typeof e;
          return (
            "function" ==
              typeof (t =
                null == e || ("object" != n && "function" != n) ? t : e.then) &&
            t
          );
        }
        function d() {
          for (var e, t, n = 0; n < this.chain.length; n++) {
            r = void 0;
            o = void 0;
            i = void 0;
            e = void 0;
            t = void 0;
            var r = this;
            var o =
              1 === this.state ? this.chain[n].success : this.chain[n].failure;
            var i = this.chain[n];
            try {
              !1 === o
                ? i.reject(r.msg)
                : (e = !0 === o ? r.msg : o.call(void 0, r.msg)) === i.promise
                  ? i.reject(TypeError("Promise-chain cycle"))
                  : (t = f(e))
                    ? t.call(e, i.resolve, i.reject)
                    : i.resolve(e);
            } catch (r) {
              i.reject(r);
            }
          }
          this.chain.length = 0;
        }
        function h(e) {
          var n,
            r = this;
          if (!r.triggered) {
            ((r.triggered = !0), r.def && (r = r.def));
            try {
              (n = f(e))
                ? s(function () {
                    var t = new v(r);
                    try {
                      n.call(
                        e,
                        function () {
                          h.apply(t, arguments);
                        },
                        function () {
                          p.apply(t, arguments);
                        },
                      );
                    } catch (e) {
                      p.call(t, e);
                    }
                  })
                : ((r.msg = e), (r.state = 1), 0 < r.chain.length && s(d, r));
            } catch (e) {
              p.call(new v(r), e);
            }
          }
        }
        function p(e) {
          var t = this;
          t.triggered ||
            ((t.triggered = !0),
            ((t = t.def ? t.def : t).msg = e),
            (t.state = 2),
            0 < t.chain.length && s(d, t));
        }
        function y(e, n, r, o) {
          for (var t = 0; t < n.length; t++)
            ((t) => {
              e.resolve(n[t]).then(function (e) {
                r(t, e);
              }, o);
            })(t);
        }
        function v(e) {
          ((this.def = e), (this.triggered = !1));
        }
        function t(e) {
          ((this.promise = e),
            (this.state = 0),
            (this.triggered = !1),
            (this.chain = []),
            (this.msg = void 0));
        }
        function m(e) {
          if ("function" != typeof e) throw TypeError("Not a function");
          if (0 !== this.__NPO__) throw TypeError("Not a promise");
          this.__NPO__ = 1;
          var r = new t(this);
          ((this.then = function (e, t) {
            var n = {
              success: "function" != typeof e || e,
              failure: "function" == typeof t && t,
            };
            return (
              (n.promise = new this.constructor(function (e, t) {
                if ("function" != typeof e || "function" != typeof t)
                  throw TypeError("Not a function");
                ((n.resolve = e), (n.reject = t));
              })),
              r.chain.push(n),
              0 !== r.state && s(d, r),
              n.promise
            );
          }),
            (this.catch = function (e) {
              return this.then(void 0, e);
            }));
          try {
            e.call(
              void 0,
              function (e) {
                h.call(r, e);
              },
              function (e) {
                p.call(r, e);
              },
            );
          } catch (e) {
            p.call(r, e);
          }
        }
        var g = e(
          {},
          "constructor",
          m,
          !(r = {
            add: function (e, t) {
              ((a = new l(e, t)), i ? (i.next = a) : (o = a), (i = a));
            },
            drain: function () {
              var e = o;
              for (o = i = n = void 0; e; ) (e.fn.call(e.self), (e = e.next));
            },
          }),
        );
        return (
          e((m.prototype = g), "__NPO__", 0, !1),
          e(m, "resolve", function (n) {
            return n && "object" == typeof n && 1 === n.__NPO__
              ? n
              : new this(function (e, t) {
                  if ("function" != typeof e || "function" != typeof t)
                    throw TypeError("Not a function");
                  e(n);
                });
          }),
          e(m, "reject", function (n) {
            return new this(function (e, t) {
              if ("function" != typeof e || "function" != typeof t)
                throw TypeError("Not a function");
              t(n);
            });
          }),
          e(m, "all", function (t) {
            var a = this;
            return "[object Array]" != u.call(t)
              ? a.reject(TypeError("Not an array"))
              : 0 === t.length
                ? a.resolve([])
                : new a(function (n, e) {
                    if ("function" != typeof n || "function" != typeof e)
                      throw TypeError("Not a function");
                    var r = t.length,
                      o = Array(r),
                      i = 0;
                    y(
                      a,
                      t,
                      function (e, t) {
                        ((o[e] = t), ++i === r && n(o));
                      },
                      e,
                    );
                  });
          }),
          e(m, "race", function (t) {
            var r = this;
            return "[object Array]" != u.call(t)
              ? r.reject(TypeError("Not an array"))
              : new r(function (n, e) {
                  if ("function" != typeof n || "function" != typeof e)
                    throw TypeError("Not a function");
                  y(
                    r,
                    t,
                    function (e, t) {
                      n(t);
                    },
                    e,
                  );
                });
          }),
          m
        );
      })()),
    m.exports && (m.exports = e[g]));
  var m,
    g,
    w = t.exports,
    b = new WeakMap();
  function k(e, t, n) {
    var r = b.get(e.element) || {};
    (t in r || (r[t] = []), r[t].push(n), b.set(e.element, r));
  }
  function E(e, t) {
    return (b.get(e.element) || {})[t] || [];
  }
  function x(e, t, n) {
    var r = b.get(e.element) || {};
    return (
      !r[t] ||
      (n
        ? (-1 !== (n = r[t].indexOf(n)) && r[t].splice(n, 1),
          b.set(e.element, r),
          r[t] && 0 === r[t].length)
        : ((r[t] = []), b.set(e.element, r), 1))
    );
  }
  function T(e) {
    if ("string" == typeof e)
      try {
        e = JSON.parse(e);
      } catch (e) {
        return (console.warn(e), {});
      }
    return e;
  }
  function j(e, t, n) {
    e.element.contentWindow &&
      e.element.contentWindow.postMessage &&
      ((t = { method: t }),
      void 0 !== n && (t.value = n),
      8 <=
        (n = parseFloat(
          navigator.userAgent.toLowerCase().replace(/^.*msie (\d+).*$/, "$1"),
        )) &&
        n < 10 &&
        (t = JSON.stringify(t)),
      e.element.contentWindow.postMessage(t, e.origin));
  }
  var X = [
    "airplay",
    "audio_tracks",
    "autopause",
    "autoplay",
    "background",
    "byline",
    "cc",
    "chapter_id",
    "chapters",
    "chromecast",
    "color",
    "colors",
    "controls",
    "dnt",
    "end_time",
    "fullscreen",
    "height",
    "id",
    "interactive_params",
    "keyboard",
    "loop",
    "maxheight",
    "maxwidth",
    "muted",
    "play_button_position",
    "playsinline",
    "portrait",
    "progress_bar",
    "quality_selector",
    "responsive",
    "speed",
    "start_time",
    "texttrack",
    "title",
    "transcript",
    "transparent",
    "unmute_button",
    "url",
    "vimeo_logo",
    "volume",
    "watch_full_video",
    "width",
  ];
  function $(r, e) {
    return X.reduce(
      function (e, t) {
        var n = r.getAttribute("data-vimeo-".concat(t));
        return ((!n && "" !== n) || (e[t] = "" === n ? 1 : n), e);
      },
      1 < arguments.length && void 0 !== e ? e : {},
    );
  }
  function _(e, t) {
    var n,
      e = e.html;
    if (t)
      return (
        null === t.getAttribute("data-vimeo-initialized") &&
          (((n = document.createElement("div")).innerHTML = e),
          t.appendChild(n.firstChild),
          t.setAttribute("data-vimeo-initialized", "true")),
        t.querySelector("iframe")
      );
    throw new TypeError("An element must be provided");
  }
  function Z(a, e, t) {
    var u = 1 < arguments.length && void 0 !== e ? e : {},
      c = 2 < arguments.length ? t : void 0;
    return new Promise(function (t, n) {
      if (!y(a))
        throw new TypeError("“".concat(a, "” is not a vimeo.com url."));
      var e,
        r = (() => {
          for (
            var e = (a || "").match(/^(?:https?:)?(?:\/\/)?([^/?]+)/),
              t = ((e && e[1]) || "").replace("player.", ""),
              n = 0,
              r = [".videoji.hk", ".vimeo.work", ".videoji.cn"];
            n < r.length;
            n++
          )
            if (t.endsWith(r[n])) return t;
          return "vimeo.com";
        })(),
        o = "https://"
          .concat(r, "/api/oembed.json?url=")
          .concat(encodeURIComponent(a));
      for (e in u)
        u.hasOwnProperty(e) &&
          (o += "&".concat(e, "=").concat(encodeURIComponent(u[e])));
      var i = new (
        "XDomainRequest" in window ? XDomainRequest : XMLHttpRequest
      )();
      (i.open("GET", o, !0),
        (i.onload = function () {
          if (404 !== i.status)
            if (403 !== i.status)
              try {
                var e = JSON.parse(i.responseText);
                403 === e.domain_status_code
                  ? (_(e, c),
                    n(new Error("“".concat(a, "” is not embeddable."))))
                  : t(e);
              } catch (e) {
                n(e);
              }
            else n(new Error("“".concat(a, "” is not embeddable.")));
          else n(new Error("“".concat(a, "” was not found.")));
        }),
        (i.onerror = function () {
          var e = i.status ? " (".concat(i.status, ")") : "";
          n(
            new Error(
              "There was an error fetching the embed code from Vimeo".concat(
                e,
                ".",
              ),
            ),
          );
        }),
        i.send());
    });
  }
  var O,
    M,
    S,
    K,
    ee = {
      role: "viewer",
      autoPlayMuted: !0,
      allowedDrift: 0.3,
      maxAllowedDrift: 1,
      minCheckInterval: 0.1,
      maxRateAdjustment: 0.2,
      maxTimeToCatchUp: 1,
    },
    te = (() => {
      var e = u,
        t = V(EventTarget);
      if ("function" != typeof t && null !== t)
        throw new TypeError(
          "Super expression must either be null or a function",
        );
      ((e.prototype = Object.create(t && t.prototype, {
        constructor: { value: e, writable: !0, configurable: !0 },
      })),
        Object.defineProperty(e, "prototype", { writable: !1 }),
        t && h(e, t));
      var r,
        n,
        o,
        a = D(u);
      function u(e, t) {
        var o,
          n,
          r =
            2 < arguments.length && void 0 !== arguments[2] ? arguments[2] : {},
          i = 3 < arguments.length ? arguments[3] : void 0;
        return (
          s(this, u),
          f(p((o = a.call(this))), "logger", void 0),
          f(p(o), "speedAdjustment", 0),
          f(
            p(o),
            "adjustSpeed",
            ((n = d(
              P().mark(function e(t, n) {
                var r;
                return P().wrap(function (e) {
                  for (;;)
                    switch ((e.prev = e.next)) {
                      case 0:
                        if (o.speedAdjustment === n) return e.abrupt("return");
                        e.next = 2;
                        break;
                      case 2:
                        return ((e.next = 4), t.getPlaybackRate());
                      case 4:
                        return (
                          (e.t0 = e.sent),
                          (e.t1 = o.speedAdjustment),
                          (e.t2 = e.t0 - e.t1),
                          (e.t3 = n),
                          (r = e.t2 + e.t3),
                          o.log("New playbackRate:  ".concat(r)),
                          (e.next = 12),
                          t.setPlaybackRate(r)
                        );
                      case 12:
                        o.speedAdjustment = n;
                      case 13:
                      case "end":
                        return e.stop();
                    }
                }, e);
              }),
            )),
            function (e, t) {
              return n.apply(this, arguments);
            }),
          ),
          (o.logger = i),
          o.init(t, e, c(c({}, ee), r)),
          o
        );
      }
      return (
        R(u, [
          {
            key: "disconnect",
            value: function () {
              this.dispatchEvent(new Event("disconnect"));
            },
          },
          {
            key: "init",
            value:
              ((o = d(
                P().mark(function e(t, n, r) {
                  var o,
                    i,
                    a,
                    u = this;
                  return P().wrap(
                    function (e) {
                      for (;;)
                        switch ((e.prev = e.next)) {
                          case 0:
                            return (
                              (e.next = 2),
                              this.waitForTOReadyState(t, "open")
                            );
                          case 2:
                            if ("viewer" === r.role)
                              return ((e.next = 5), this.updatePlayer(t, n, r));
                            e.next = 10;
                            break;
                          case 5:
                            ((o = H(t, "change", function () {
                              return u.updatePlayer(t, n, r);
                            })),
                              (i = this.maintainPlaybackPosition(t, n, r)),
                              this.addEventListener("disconnect", function () {
                                (i.cancel(), o.cancel());
                              }),
                              (e.next = 14));
                            break;
                          case 10:
                            return (
                              (e.next = 12),
                              this.updateTimingObject(t, n)
                            );
                          case 12:
                            ((a = H(
                              n,
                              ["seeked", "play", "pause", "ratechange"],
                              function () {
                                return u.updateTimingObject(t, n);
                              },
                              "on",
                              "off",
                            )),
                              this.addEventListener("disconnect", function () {
                                return a.cancel();
                              }));
                          case 14:
                          case "end":
                            return e.stop();
                        }
                    },
                    e,
                    this,
                  );
                }),
              )),
              function (e, t, n) {
                return o.apply(this, arguments);
              }),
          },
          {
            key: "updateTimingObject",
            value:
              ((n = d(
                P().mark(function e(t, n) {
                  return P().wrap(function (e) {
                    for (;;)
                      switch ((e.prev = e.next)) {
                        case 0:
                          return ((e.t0 = t), (e.next = 3), n.getCurrentTime());
                        case 3:
                          return ((e.t1 = e.sent), (e.next = 6), n.getPaused());
                        case 6:
                          e.sent ? ((e.t2 = 0), (e.next = 13)) : (e.next = 10);
                          break;
                        case 10:
                          return ((e.next = 12), n.getPlaybackRate());
                        case 12:
                          e.t2 = e.sent;
                        case 13:
                          ((e.t3 = e.t2),
                            (e.t4 = { position: e.t1, velocity: e.t3 }),
                            e.t0.update.call(e.t0, e.t4));
                        case 16:
                        case "end":
                          return e.stop();
                      }
                  }, e);
                }),
              )),
              function (e, t) {
                return n.apply(this, arguments);
              }),
          },
          {
            key: "updatePlayer",
            value:
              ((r = d(
                P().mark(function e(t, n, r) {
                  var o, i;
                  return P().wrap(
                    function (e) {
                      for (;;)
                        switch ((e.prev = e.next)) {
                          case 0:
                            if (
                              ((i = t.query()),
                              (o = i.position),
                              (i = i.velocity),
                              "number" == typeof o && n.setCurrentTime(o),
                              "number" != typeof i)
                            )
                              e.next = 25;
                            else {
                              if (0 === i) return ((e.next = 6), n.getPaused());
                              e.next = 11;
                            }
                            break;
                          case 6:
                            if (((e.t0 = e.sent), !1 !== e.t0)) {
                              e.next = 9;
                              break;
                            }
                            n.pause();
                          case 9:
                            e.next = 25;
                            break;
                          case 11:
                            if (0 < i) return ((e.next = 14), n.getPaused());
                            e.next = 25;
                            break;
                          case 14:
                            if (((e.t1 = e.sent), !0 === e.t1))
                              return (
                                (e.next = 18),
                                n.play().catch(
                                  (() => {
                                    var t = d(
                                      P().mark(function e(t) {
                                        return P().wrap(function (e) {
                                          for (;;)
                                            switch ((e.prev = e.next)) {
                                              case 0:
                                                if (
                                                  "NotAllowedError" ===
                                                    t.name &&
                                                  r.autoPlayMuted
                                                )
                                                  return (
                                                    (e.next = 3),
                                                    n.setMuted(!0)
                                                  );
                                                e.next = 5;
                                                break;
                                              case 3:
                                                return (
                                                  (e.next = 5),
                                                  n.play().catch(function (e) {
                                                    return console.error(
                                                      "Couldn't play the video from TimingSrcConnector. Error:",
                                                      e,
                                                    );
                                                  })
                                                );
                                              case 5:
                                              case "end":
                                                return e.stop();
                                            }
                                        }, e);
                                      }),
                                    );
                                    return function (e) {
                                      return t.apply(this, arguments);
                                    };
                                  })(),
                                )
                              );
                            e.next = 19;
                            break;
                          case 18:
                            this.updatePlayer(t, n, r);
                          case 19:
                            return ((e.next = 21), n.getPlaybackRate());
                          case 21:
                            if (((e.t2 = e.sent), (e.t3 = i), e.t2 === e.t3)) {
                              e.next = 25;
                              break;
                            }
                            n.setPlaybackRate(i);
                          case 25:
                          case "end":
                            return e.stop();
                        }
                    },
                    e,
                    this,
                  );
                }),
              )),
              function (e, t, n) {
                return r.apply(this, arguments);
              }),
          },
          {
            key: "maintainPlaybackPosition",
            value: function (i, a, e) {
              var t,
                u = this,
                c = e.allowedDrift,
                l = e.maxAllowedDrift,
                s = e.maxRateAdjustment,
                f = e.maxTimeToCatchUp,
                e = 1e3 * Math.min(f, Math.max(e.minCheckInterval, l)),
                n =
                  ((t = d(
                    P().mark(function e() {
                      var t, n, r, o;
                      return P().wrap(function (e) {
                        for (;;)
                          switch ((e.prev = e.next)) {
                            case 0:
                              if (((e.t0 = 0 === i.query().velocity), e.t0)) {
                                e.next = 6;
                                break;
                              }
                              return ((e.next = 4), a.getPaused());
                            case 4:
                              ((e.t1 = e.sent), (e.t0 = !0 === e.t1));
                            case 6:
                              if (e.t0) return e.abrupt("return");
                              e.next = 8;
                              break;
                            case 8:
                              return (
                                (e.t2 = i.query().position),
                                (e.next = 11),
                                a.getCurrentTime()
                              );
                            case 11:
                              if (
                                ((e.t3 = e.sent),
                                (t = e.t2 - e.t3),
                                (n = Math.abs(t)),
                                u.log("Drift: ".concat(t)),
                                l < n)
                              )
                                return ((e.next = 18), u.adjustSpeed(a, 0));
                              e.next = 22;
                              break;
                            case 18:
                              (a.setCurrentTime(i.query().position),
                                u.log("Resync by currentTime"),
                                (e.next = 29));
                              break;
                            case 22:
                              if (c < n)
                                return (
                                  (o = (o = n / f) < (r = s) ? (r - o) / 2 : r),
                                  (e.next = 28),
                                  u.adjustSpeed(a, o * Math.sign(t))
                                );
                              e.next = 29;
                              break;
                            case 28:
                              u.log("Resync by playbackRate");
                            case 29:
                            case "end":
                              return e.stop();
                          }
                      }, e);
                    }),
                  )),
                  function () {
                    return t.apply(this, arguments);
                  }),
                r = setInterval(function () {
                  return n();
                }, e);
              return {
                cancel: function () {
                  return clearInterval(r);
                },
              };
            },
          },
          {
            key: "log",
            value: function (e) {
              var t;
              null != (t = this.logger) &&
                t.call(this, "TimingSrcConnector: ".concat(e));
            },
          },
          {
            key: "waitForTOReadyState",
            value: function (n, r) {
              return new Promise(function (t) {
                !(function e() {
                  n.readyState === r
                    ? t()
                    : n.addEventListener("readystatechange", e, { once: !0 });
                })();
              });
            },
          },
        ]),
        u
      );
    })(),
    C = new WeakMap(),
    F = new WeakMap(),
    N = {},
    L =
      (R(A, [
        {
          key: "callMethod",
          value: function (n, e) {
            var r = this,
              o = 1 < arguments.length && void 0 !== e ? e : {};
            return new w(function (e, t) {
              return r
                .ready()
                .then(function () {
                  (k(r, n, { resolve: e, reject: t }), j(r, n, o));
                })
                .catch(t);
            });
          },
        },
        {
          key: "get",
          value: function (n) {
            var r = this;
            return new w(function (e, t) {
              return (
                (n = U(n, "get")),
                r
                  .ready()
                  .then(function () {
                    (k(r, n, { resolve: e, reject: t }), j(r, n));
                  })
                  .catch(t)
              );
            });
          },
        },
        {
          key: "set",
          value: function (n, r) {
            var o = this;
            return new w(function (e, t) {
              if (((n = U(n, "set")), null == r))
                throw new TypeError("There must be a value to set.");
              return o
                .ready()
                .then(function () {
                  (k(o, n, { resolve: e, reject: t }), j(o, n, r));
                })
                .catch(t);
            });
          },
        },
        {
          key: "on",
          value: function (e, t) {
            if (!e) throw new TypeError("You must pass an event name.");
            if (!t) throw new TypeError("You must pass a callback function.");
            if ("function" != typeof t)
              throw new TypeError("The callback must be a function.");
            (0 === E(this, "event:".concat(e)).length &&
              this.callMethod("addEventListener", e).catch(function () {}),
              k(this, "event:".concat(e), t));
          },
        },
        {
          key: "off",
          value: function (e, t) {
            if (!e) throw new TypeError("You must pass an event name.");
            if (t && "function" != typeof t)
              throw new TypeError("The callback must be a function.");
            x(this, "event:".concat(e), t) &&
              this.callMethod("removeEventListener", e).catch(function (e) {});
          },
        },
        {
          key: "loadVideo",
          value: function (e) {
            return this.callMethod("loadVideo", e);
          },
        },
        {
          key: "ready",
          value: function () {
            var e =
              F.get(this) ||
              new w(function (e, t) {
                t(new Error("Unknown player. Probably unloaded."));
              });
            return w.resolve(e);
          },
        },
        {
          key: "addCuePoint",
          value: function (e, t) {
            return this.callMethod("addCuePoint", {
              time: e,
              data: 1 < arguments.length && void 0 !== t ? t : {},
            });
          },
        },
        {
          key: "removeCuePoint",
          value: function (e) {
            return this.callMethod("removeCuePoint", e);
          },
        },
        {
          key: "enableTextTrack",
          value: function (e, t) {
            if (e)
              return this.callMethod("enableTextTrack", {
                language: e,
                kind: t,
              });
            throw new TypeError("You must pass a language.");
          },
        },
        {
          key: "disableTextTrack",
          value: function () {
            return this.callMethod("disableTextTrack");
          },
        },
        {
          key: "pause",
          value: function () {
            return this.callMethod("pause");
          },
        },
        {
          key: "play",
          value: function () {
            return this.callMethod("play");
          },
        },
        {
          key: "requestFullscreen",
          value: function () {
            return N.isEnabled
              ? N.request(this.element)
              : this.callMethod("requestFullscreen");
          },
        },
        {
          key: "exitFullscreen",
          value: function () {
            return N.isEnabled ? N.exit() : this.callMethod("exitFullscreen");
          },
        },
        {
          key: "getFullscreen",
          value: function () {
            return N.isEnabled
              ? w.resolve(N.isFullscreen)
              : this.get("fullscreen");
          },
        },
        {
          key: "requestPictureInPicture",
          value: function () {
            return this.callMethod("requestPictureInPicture");
          },
        },
        {
          key: "exitPictureInPicture",
          value: function () {
            return this.callMethod("exitPictureInPicture");
          },
        },
        {
          key: "getPictureInPicture",
          value: function () {
            return this.get("pictureInPicture");
          },
        },
        {
          key: "remotePlaybackPrompt",
          value: function () {
            return this.callMethod("remotePlaybackPrompt");
          },
        },
        {
          key: "unload",
          value: function () {
            return this.callMethod("unload");
          },
        },
        {
          key: "destroy",
          value: function () {
            var n = this;
            return new w(function (e) {
              var t;
              (F.delete(n),
                C.delete(n.element),
                n._originalElement &&
                  (C.delete(n._originalElement),
                  n._originalElement.removeAttribute("data-vimeo-initialized")),
                n.element &&
                  "IFRAME" === n.element.nodeName &&
                  n.element.parentNode &&
                  (n.element.parentNode.parentNode &&
                  n._originalElement &&
                  n._originalElement !== n.element.parentNode
                    ? n.element.parentNode.parentNode.removeChild(
                        n.element.parentNode,
                      )
                    : n.element.parentNode.removeChild(n.element)),
                n.element &&
                  "DIV" === n.element.nodeName &&
                  n.element.parentNode &&
                  (n.element.removeAttribute("data-vimeo-initialized"),
                  (t = n.element.querySelector("iframe"))) &&
                  t.parentNode &&
                  (t.parentNode.parentNode &&
                  n._originalElement &&
                  n._originalElement !== t.parentNode
                    ? t.parentNode.parentNode.removeChild(t.parentNode)
                    : t.parentNode.removeChild(t)),
                n._window.removeEventListener("message", n._onMessage),
                N.isEnabled &&
                  N.off("fullscreenchange", n.fullscreenchangeHandler),
                e());
            });
          },
        },
        {
          key: "getAutopause",
          value: function () {
            return this.get("autopause");
          },
        },
        {
          key: "setAutopause",
          value: function (e) {
            return this.set("autopause", e);
          },
        },
        {
          key: "getBuffered",
          value: function () {
            return this.get("buffered");
          },
        },
        {
          key: "getCameraProps",
          value: function () {
            return this.get("cameraProps");
          },
        },
        {
          key: "setCameraProps",
          value: function (e) {
            return this.set("cameraProps", e);
          },
        },
        {
          key: "getChapters",
          value: function () {
            return this.get("chapters");
          },
        },
        {
          key: "getCurrentChapter",
          value: function () {
            return this.get("currentChapter");
          },
        },
        {
          key: "getColor",
          value: function () {
            return this.get("color");
          },
        },
        {
          key: "getColors",
          value: function () {
            return w.all([
              this.get("colorOne"),
              this.get("colorTwo"),
              this.get("colorThree"),
              this.get("colorFour"),
            ]);
          },
        },
        {
          key: "setColor",
          value: function (e) {
            return this.set("color", e);
          },
        },
        {
          key: "setColors",
          value: function (e) {
            var t;
            return Array.isArray(e)
              ? ((t = new w(function (e) {
                  return e(null);
                })),
                (e = [
                  e[0] ? this.set("colorOne", e[0]) : t,
                  e[1] ? this.set("colorTwo", e[1]) : t,
                  e[2] ? this.set("colorThree", e[2]) : t,
                  e[3] ? this.set("colorFour", e[3]) : t,
                ]),
                w.all(e))
              : new w(function (e, t) {
                  return t(new TypeError("Argument must be an array."));
                });
          },
        },
        {
          key: "getCuePoints",
          value: function () {
            return this.get("cuePoints");
          },
        },
        {
          key: "getCurrentTime",
          value: function () {
            return this.get("currentTime");
          },
        },
        {
          key: "setCurrentTime",
          value: function (e) {
            return this.set("currentTime", e);
          },
        },
        {
          key: "getDuration",
          value: function () {
            return this.get("duration");
          },
        },
        {
          key: "getEnded",
          value: function () {
            return this.get("ended");
          },
        },
        {
          key: "getLoop",
          value: function () {
            return this.get("loop");
          },
        },
        {
          key: "setLoop",
          value: function (e) {
            return this.set("loop", e);
          },
        },
        {
          key: "setMuted",
          value: function (e) {
            return this.set("muted", e);
          },
        },
        {
          key: "getMuted",
          value: function () {
            return this.get("muted");
          },
        },
        {
          key: "getPaused",
          value: function () {
            return this.get("paused");
          },
        },
        {
          key: "getPlaybackRate",
          value: function () {
            return this.get("playbackRate");
          },
        },
        {
          key: "setPlaybackRate",
          value: function (e) {
            return this.set("playbackRate", e);
          },
        },
        {
          key: "getPlayed",
          value: function () {
            return this.get("played");
          },
        },
        {
          key: "getQualities",
          value: function () {
            return this.get("qualities");
          },
        },
        {
          key: "getQuality",
          value: function () {
            return this.get("quality");
          },
        },
        {
          key: "setQuality",
          value: function (e) {
            return this.set("quality", e);
          },
        },
        {
          key: "getRemotePlaybackAvailability",
          value: function () {
            return this.get("remotePlaybackAvailability");
          },
        },
        {
          key: "getRemotePlaybackState",
          value: function () {
            return this.get("remotePlaybackState");
          },
        },
        {
          key: "getSeekable",
          value: function () {
            return this.get("seekable");
          },
        },
        {
          key: "getSeeking",
          value: function () {
            return this.get("seeking");
          },
        },
        {
          key: "getTextTracks",
          value: function () {
            return this.get("textTracks");
          },
        },
        {
          key: "getVideoEmbedCode",
          value: function () {
            return this.get("videoEmbedCode");
          },
        },
        {
          key: "getVideoId",
          value: function () {
            return this.get("videoId");
          },
        },
        {
          key: "getVideoTitle",
          value: function () {
            return this.get("videoTitle");
          },
        },
        {
          key: "getVideoWidth",
          value: function () {
            return this.get("videoWidth");
          },
        },
        {
          key: "getVideoHeight",
          value: function () {
            return this.get("videoHeight");
          },
        },
        {
          key: "getVideoUrl",
          value: function () {
            return this.get("videoUrl");
          },
        },
        {
          key: "getVolume",
          value: function () {
            return this.get("volume");
          },
        },
        {
          key: "setVolume",
          value: function (e) {
            return this.set("volume", e);
          },
        },
        {
          key: "setTimingSrc",
          value:
            ((K = d(
              P().mark(function e(t, n) {
                var r,
                  o = this;
                return P().wrap(
                  function (e) {
                    for (;;)
                      switch ((e.prev = e.next)) {
                        case 0:
                          if (t) {
                            e.next = 2;
                            break;
                          }
                          throw new TypeError(
                            "A Timing Object must be provided.",
                          );
                        case 2:
                          return ((e.next = 4), this.ready());
                        case 4:
                          return (
                            (r = new te(this, t, n)),
                            j(this, "notifyTimingObjectConnect"),
                            r.addEventListener("disconnect", function () {
                              return j(o, "notifyTimingObjectDisconnect");
                            }),
                            e.abrupt("return", r)
                          );
                        case 8:
                        case "end":
                          return e.stop();
                      }
                  },
                  e,
                  this,
                );
              }),
            )),
            function (e, t) {
              return K.apply(this, arguments);
            }),
        },
      ]),
      A);
  function A(o) {
    var l = this,
      t = 1 < arguments.length && void 0 !== arguments[1] ? arguments[1] : {};
    if (
      (s(this, A),
      window.jQuery &&
        o instanceof jQuery &&
        (1 < o.length &&
          window.console &&
          console.warn &&
          console.warn(
            "A jQuery object with multiple elements was passed, using the first element.",
          ),
        (o = o[0])),
      (n = o =
        "undefined" != typeof document && "string" == typeof o
          ? document.getElementById(o)
          : o),
      !Boolean(
        n &&
          1 === n.nodeType &&
          "nodeName" in n &&
          n.ownerDocument &&
          n.ownerDocument.defaultView,
      ))
    )
      throw new TypeError(
        "You must pass either a valid element or a valid id.",
      );
    if (
      "IFRAME" ===
        (o = "IFRAME" !== o.nodeName && (n = o.querySelector("iframe")) ? n : o)
          .nodeName &&
      !y(o.getAttribute("src") || "")
    )
      throw new Error("The player element passed isn’t a Vimeo embed.");
    if (C.has(o)) return C.get(o);
    ((this._window = o.ownerDocument.defaultView),
      (this.element = o),
      (this.origin = "*"));
    var e,
      n = new w(function (u, c) {
        var e;
        ((l._onMessage = function (e) {
          var n, r, t, o, i, a;
          y(e.origin) &&
            l.element.contentWindow === e.source &&
            ("*" === l.origin && (l.origin = e.origin),
            (e = T(e.data)) &&
            "error" === e.event &&
            e.data &&
            "ready" === e.data.method
              ? (((a = new Error(e.data.message)).name = e.data.name), c(a))
              : ((a = e && "ready" === e.event),
                (o = e && "ping" === e.method),
                a || o
                  ? (l.element.setAttribute("data-ready", "true"), u())
                  : ((n = l),
                    (a = []),
                    (r = T((r = e))).event
                      ? ("error" === r.event &&
                          E(n, r.data.method).forEach(function (e) {
                            var t = new Error(r.data.message);
                            ((t.name = r.data.name),
                              e.reject(t),
                              x(n, r.data.method, e));
                          }),
                        (a = E(n, "event:".concat(r.event))),
                        (t = r.data))
                      : r.method &&
                        ((o = n),
                        (e = r.method),
                        (e =
                          !((i = E(o, e)).length < 1) &&
                          (x(o, e, (o = i.shift())), o))) &&
                        (a.push(e), (t = r.value)),
                    a.forEach(function (e) {
                      try {
                        "function" == typeof e ? e.call(n, t) : e.resolve(t);
                      } catch (e) {}
                    }))));
        }),
          l._window.addEventListener("message", l._onMessage),
          "IFRAME" !== l.element.nodeName &&
            Z(B((e = $(o, t))), e, o)
              .then(function (e) {
                var t,
                  n,
                  r = _(e, o);
                return (
                  (l.element = r),
                  (t = l._originalElement = o),
                  (r = r),
                  (n = b.get(t)),
                  b.set(r, n),
                  b.delete(t),
                  C.set(l.element, l),
                  e
                );
              })
              .catch(c));
      });
    return (
      F.set(this, n),
      C.set(this.element, this),
      "IFRAME" === this.element.nodeName && j(this, "ping"),
      N.isEnabled &&
        ((e = function () {
          return N.exit();
        }),
        (this.fullscreenchangeHandler = function () {
          ((N.isFullscreen ? k : x)(l, "event:exitFullscreen", e),
            l.ready().then(function () {
              j(l, "fullscreenchange", N.isFullscreen);
            }));
        }),
        N.on("fullscreenchange", this.fullscreenchangeHandler)),
      this
    );
  }
  return (
    z ||
      ((O = (() => {
        for (
          var e,
            t = [
              [
                "requestFullscreen",
                "exitFullscreen",
                "fullscreenElement",
                "fullscreenEnabled",
                "fullscreenchange",
                "fullscreenerror",
              ],
              [
                "webkitRequestFullscreen",
                "webkitExitFullscreen",
                "webkitFullscreenElement",
                "webkitFullscreenEnabled",
                "webkitfullscreenchange",
                "webkitfullscreenerror",
              ],
              [
                "webkitRequestFullScreen",
                "webkitCancelFullScreen",
                "webkitCurrentFullScreenElement",
                "webkitCancelFullScreen",
                "webkitfullscreenchange",
                "webkitfullscreenerror",
              ],
              [
                "mozRequestFullScreen",
                "mozCancelFullScreen",
                "mozFullScreenElement",
                "mozFullScreenEnabled",
                "mozfullscreenchange",
                "mozfullscreenerror",
              ],
              [
                "msRequestFullscreen",
                "msExitFullscreen",
                "msFullscreenElement",
                "msFullscreenEnabled",
                "MSFullscreenChange",
                "MSFullscreenError",
              ],
            ],
            n = 0,
            r = t.length,
            o = {};
          n < r;
          n++
        )
          if ((e = t[n]) && e[1] in document) {
            for (n = 0; n < e.length; n++) o[t[0][n]] = e[n];
            return o;
          }
        return !1;
      })()),
      (M = {
        fullscreenchange: O.fullscreenchange,
        fullscreenerror: O.fullscreenerror,
      }),
      (S = {
        request: function (o) {
          return new Promise(function (e, t) {
            function n() {
              (S.off("fullscreenchange", n), e());
            }
            S.on("fullscreenchange", n);
            var r = (o = o || document.documentElement)[O.requestFullscreen]();
            r instanceof Promise && r.then(n).catch(t);
          });
        },
        exit: function () {
          return new Promise(function (t, e) {
            var n, r;
            S.isFullscreen
              ? (S.on(
                  "fullscreenchange",
                  (n = function e() {
                    (S.off("fullscreenchange", e), t());
                  }),
                ),
                (r = document[O.exitFullscreen]()) instanceof Promise &&
                  r.then(n).catch(e))
              : t();
          });
        },
        on: function (e, t) {
          e = M[e];
          e && document.addEventListener(e, t);
        },
        off: function (e, t) {
          e = M[e];
          e && document.removeEventListener(e, t);
        },
      }),
      Object.defineProperties(S, {
        isFullscreen: {
          get: function () {
            return Boolean(document[O.fullscreenElement]);
          },
        },
        element: {
          enumerable: !0,
          get: function () {
            return document[O.fullscreenElement];
          },
        },
        isEnabled: {
          enumerable: !0,
          get: function () {
            return Boolean(document[O.fullscreenEnabled]);
          },
        },
      }),
      (N = S),
      (function () {
        function n(e) {
          "console" in window &&
            console.error &&
            console.error("There was an error creating an embed: ".concat(e));
        }
        var e = document;
        [].slice
          .call(e.querySelectorAll("[data-vimeo-id], [data-vimeo-url]"))
          .forEach(function (t) {
            try {
              var e;
              null === t.getAttribute("data-vimeo-defer") &&
                Z(B((e = $(t))), e, t)
                  .then(function (e) {
                    return _(e, t);
                  })
                  .catch(n);
            } catch (e) {
              n(e);
            }
          });
      })(),
      (function () {
        var r = document;
        window.VimeoPlayerResizeEmbeds_ ||
          ((window.VimeoPlayerResizeEmbeds_ = !0),
          window.addEventListener("message", function (e) {
            if (y(e.origin) && e.data && "spacechange" === e.data.event)
              for (
                var t = r.querySelectorAll("iframe"), n = 0;
                n < t.length;
                n++
              )
                if (t[n].contentWindow === e.source) {
                  t[n].parentElement.style.paddingBottom = "".concat(
                    e.data.data[0].bottom,
                    "px",
                  );
                  break;
                }
          }));
      })(),
      (function () {
        var a = document;
        window.VimeoSeoMetadataAppended ||
          ((window.VimeoSeoMetadataAppended = !0),
          window.addEventListener("message", function (e) {
            if (y(e.origin)) {
              var t = T(e.data);
              if (t && "ready" === t.event)
                for (
                  var n = a.querySelectorAll("iframe"), r = 0;
                  r < n.length;
                  r++
                ) {
                  var o = n[r],
                    i = o.contentWindow === e.source;
                  G(o.src) &&
                    i &&
                    new L(o).callMethod(
                      "appendVideoMetadata",
                      window.location.href,
                    );
                }
            }
          }));
      })(),
      (function () {
        var a,
          t = document;
        window.VimeoCheckedUrlTimeParam ||
          ((window.VimeoCheckedUrlTimeParam = !0),
          (a = function (e) {
            "console" in window &&
              console.error &&
              console.error("There was an error getting video Id: ".concat(e));
          }),
          window.addEventListener("message", function (r) {
            if (y(r.origin)) {
              var e = T(r.data);
              if (e && "ready" === e.event)
                for (
                  var o = t.querySelectorAll("iframe"), i = 0;
                  i < o.length;
                  i++
                )
                  (() => {
                    var t,
                      e = o[i],
                      n = e.contentWindow === r.source;
                    G(e.src) &&
                      n &&
                      (t = new L(e))
                        .getVideoId()
                        .then(function (e) {
                          var e = new RegExp(
                            "[?&]vimeo_t_".concat(e, "=([^&#]*)"),
                          ).exec(window.location.href);
                          e &&
                            e[1] &&
                            ((e = decodeURI(e[1])), t.setCurrentTime(e));
                        })
                        .catch(a);
                  })();
            }
          }));
      })()),
    L
  );
});
var _self =
    "undefined" != typeof window
      ? window
      : "undefined" != typeof WorkerGlobalScope &&
          self instanceof WorkerGlobalScope
        ? self
        : {},
  Prism = ((l) => {
    var e,
      n = /(?:^|\s)lang(?:uage)?-([\w-]+)(?=\s|$)/i,
      t = 0,
      a = {},
      F = {
        manual: l.Prism && l.Prism.manual,
        disableWorkerMessageHandler:
          l.Prism && l.Prism.disableWorkerMessageHandler,
        util: {
          encode: function e(t) {
            return t instanceof _
              ? new _(t.type, e(t.content), t.alias)
              : Array.isArray(t)
                ? t.map(e)
                : t
                    .replace(/&/g, "&amp;")
                    .replace(/</g, "&lt;")
                    .replace(/\u00a0/g, " ");
          },
          type: function (e) {
            return Object.prototype.toString.call(e).slice(8, -1);
          },
          objId: function (e) {
            return (
              e.__id || Object.defineProperty(e, "__id", { value: ++t }),
              e.__id
            );
          },
          clone: function n(e, a) {
            var r, t;
            switch (((a = a || {}), F.util.type(e))) {
              case "Object":
                if (((t = F.util.objId(e)), a[t])) return a[t];
                for (var i in ((r = {}), (a[t] = r), e))
                  e.hasOwnProperty(i) && (r[i] = n(e[i], a));
                return r;
              case "Array":
                return ((t = F.util.objId(e)), a[t])
                  ? a[t]
                  : ((r = []),
                    (a[t] = r),
                    e.forEach(function (e, t) {
                      r[t] = n(e, a);
                    }),
                    r);
              default:
                return e;
            }
          },
          getLanguage: function (e) {
            for (; e; ) {
              var t = n.exec(e.className);
              if (t) return t[1].toLowerCase();
              e = e.parentElement;
            }
            return "none";
          },
          setLanguage: function (e, t) {
            ((e.className = e.className.replace(RegExp(n, "gi"), "")),
              e.classList.add("language-" + t));
          },
          currentScript: function () {
            if ("undefined" == typeof document) return null;
            if ("currentScript" in document) return document.currentScript;
            try {
              throw new Error();
            } catch (e) {
              var t = (/at [^(\r\n]*\((.*):[^:]+:[^:]+\)$/i.exec(e.stack) ||
                [])[1];
              if (t) {
                var n,
                  a = document.getElementsByTagName("script");
                for (n in a) if (a[n].src == t) return a[n];
              }
              return null;
            }
          },
          isActive: function (e, t, n) {
            for (var a = "no-" + t; e; ) {
              var r = e.classList;
              if (r.contains(t)) return !0;
              if (r.contains(a)) return !1;
              e = e.parentElement;
            }
            return !!n;
          },
        },
        languages: {
          plain: a,
          plaintext: a,
          text: a,
          txt: a,
          extend: function (e, t) {
            var n,
              a = F.util.clone(F.languages[e]);
            for (n in t) a[n] = t[n];
            return a;
          },
          insertBefore: function (n, e, t, a) {
            var r,
              i = (a = a || F.languages)[n],
              s = {};
            for (r in i)
              if (i.hasOwnProperty(r)) {
                if (r == e)
                  for (var o in t) t.hasOwnProperty(o) && (s[o] = t[o]);
                t.hasOwnProperty(r) || (s[r] = i[r]);
              }
            var l = a[n];
            return (
              (a[n] = s),
              F.languages.DFS(F.languages, function (e, t) {
                t === l && e != n && (this[e] = s);
              }),
              s
            );
          },
          DFS: function e(t, n, a, r) {
            r = r || {};
            var i,
              s,
              o,
              l = F.util.objId;
            for (i in t)
              t.hasOwnProperty(i) &&
                (n.call(t, i, t[i], a || i),
                (s = t[i]),
                "Object" !== (o = F.util.type(s)) || r[l(s)]
                  ? "Array" !== o || r[l(s)] || ((r[l(s)] = !0), e(s, n, i, r))
                  : ((r[l(s)] = !0), e(s, n, null, r)));
          },
        },
        plugins: {},
        highlightAll: function (e, t) {
          F.highlightAllUnder(document, e, t);
        },
        highlightAllUnder: function (e, t, n) {
          var a = {
            callback: n,
            container: e,
            selector:
              'code[class*="language-"], [class*="language-"] code, code[class*="lang-"], [class*="lang-"] code',
          };
          (F.hooks.run("before-highlightall", a),
            (a.elements = Array.prototype.slice.apply(
              a.container.querySelectorAll(a.selector),
            )),
            F.hooks.run("before-all-elements-highlight", a));
          for (var r, i = 0; (r = a.elements[i++]); )
            F.highlightElement(r, !0 === t, a.callback);
        },
        highlightElement: function (e, t, n) {
          var a = F.util.getLanguage(e),
            r = F.languages[a],
            i = (F.util.setLanguage(e, a), e.parentElement);
          i && "pre" === i.nodeName.toLowerCase() && F.util.setLanguage(i, a);
          var s = { element: e, language: a, grammar: r, code: e.textContent };
          function o(e) {
            ((s.highlightedCode = e),
              F.hooks.run("before-insert", s),
              (s.element.innerHTML = s.highlightedCode),
              F.hooks.run("after-highlight", s),
              F.hooks.run("complete", s),
              n && n.call(s.element));
          }
          (F.hooks.run("before-sanity-check", s),
            (i = s.element.parentElement) &&
              "pre" === i.nodeName.toLowerCase() &&
              !i.hasAttribute("tabindex") &&
              i.setAttribute("tabindex", "0"),
            s.code
              ? (F.hooks.run("before-highlight", s),
                s.grammar
                  ? t && l.Worker
                    ? (((a = new Worker(F.filename)).onmessage = function (e) {
                        o(e.data);
                      }),
                      a.postMessage(
                        JSON.stringify({
                          language: s.language,
                          code: s.code,
                          immediateClose: !0,
                        }),
                      ))
                    : o(F.highlight(s.code, s.grammar, s.language))
                  : o(F.util.encode(s.code)))
              : (F.hooks.run("complete", s), n && n.call(s.element)));
        },
        highlight: function (e, t, n) {
          e = { code: e, grammar: t, language: n };
          if ((F.hooks.run("before-tokenize", e), e.grammar))
            return (
              (e.tokens = F.tokenize(e.code, e.grammar)),
              F.hooks.run("after-tokenize", e),
              _.stringify(F.util.encode(e.tokens), e.language)
            );
          throw new Error('The language "' + e.language + '" has no grammar.');
        },
        tokenize: function (e, t) {
          var n = t.rest;
          if (n) {
            for (var a in n) t[a] = n[a];
            delete t.rest;
          }
          for (
            var r = new u(),
              i =
                (R(r, r.head, e),
                !(function e(t, n, a, r, i, s) {
                  for (var o in a)
                    if (a.hasOwnProperty(o) && a[o]) {
                      var l = a[o];
                      l = Array.isArray(l) ? l : [l];
                      for (var u = 0; u < l.length; ++u) {
                        if (s && s.cause == o + "," + u) return;
                        for (
                          var c,
                            p = l[u],
                            d = p.inside,
                            g = !!p.lookbehind,
                            m = !!p.greedy,
                            h = p.alias,
                            f =
                              (m &&
                                !p.pattern.global &&
                                ((c = p.pattern
                                  .toString()
                                  .match(/[imsuy]*$/)[0]),
                                (p.pattern = RegExp(
                                  p.pattern.source,
                                  c + "g",
                                ))),
                              p.pattern || p),
                            b = r.next,
                            y = i;
                          b !== n.tail && !(s && y >= s.reach);
                          y += b.value.length, b = b.next
                        ) {
                          var S = b.value;
                          if (n.length > t.length) return;
                          if (!(S instanceof _)) {
                            var k,
                              A = 1;
                            if (m) {
                              if (!(k = N(f, y, t, g)) || k.index >= t.length)
                                break;
                              var v = k.index,
                                w = k.index + k[0].length,
                                P = y;
                              for (P += b.value.length; P <= v; )
                                ((b = b.next), (P += b.value.length));
                              if (
                                ((P -= b.value.length),
                                (y = P),
                                b.value instanceof _)
                              )
                                continue;
                              for (
                                var x = b;
                                x !== n.tail &&
                                (P < w || "string" == typeof x.value);
                                x = x.next
                              )
                                (A++, (P += x.value.length));
                              (A--, (S = t.slice(y, P)), (k.index -= y));
                            } else if (!(k = N(f, 0, S, g))) continue;
                            var v = k.index,
                              E = k[0],
                              L = S.slice(0, v),
                              T = S.slice(v + E.length),
                              S = y + S.length,
                              C = (s && S > s.reach && (s.reach = S), b.prev),
                              L =
                                (L && ((C = R(n, C, L)), (y += L.length)),
                                O(n, C, A),
                                new _(o, d ? F.tokenize(E, d) : E, h, E));
                            ((b = R(n, C, L)),
                              T && R(n, b, T),
                              1 < A &&
                                ((E = { cause: o + "," + u, reach: S }),
                                e(t, n, a, b.prev, y, E),
                                s) &&
                                E.reach > s.reach &&
                                (s.reach = E.reach));
                          }
                        }
                      }
                    }
                })(e, r, t, r.head, 0),
                r),
              s = [],
              o = i.head.next;
            o !== i.tail;

          )
            (s.push(o.value), (o = o.next));
          return s;
        },
        hooks: {
          all: {},
          add: function (e, t) {
            var n = F.hooks.all;
            ((n[e] = n[e] || []), n[e].push(t));
          },
          run: function (e, t) {
            var n = F.hooks.all[e];
            if (n && n.length) for (var a, r = 0; (a = n[r++]); ) a(t);
          },
        },
        Token: _,
      };
    function _(e, t, n, a) {
      ((this.type = e),
        (this.content = t),
        (this.alias = n),
        (this.length = 0 | (a || "").length));
    }
    function N(e, t, n, a) {
      e.lastIndex = t;
      t = e.exec(n);
      return (
        t &&
          a &&
          t[1] &&
          ((e = t[1].length), (t.index += e), (t[0] = t[0].slice(e))),
        t
      );
    }
    function u() {
      var e = { value: null, prev: null, next: null },
        t = { value: null, prev: e, next: null };
      ((e.next = t), (this.head = e), (this.tail = t), (this.length = 0));
    }
    function R(e, t, n) {
      var a = t.next,
        n = { value: n, prev: t, next: a };
      return ((a.prev = t.next = n), e.length++, n);
    }
    function O(e, t, n) {
      for (var a = t.next, r = 0; r < n && a !== e.tail; r++) a = a.next;
      (((t.next = a).prev = t), (e.length -= r));
    }
    return (
      (l.Prism = F),
      (_.stringify = function t(e, n) {
        if ("string" == typeof e) return e;
        var a;
        if (Array.isArray(e))
          return (
            (a = ""),
            e.forEach(function (e) {
              a += t(e, n);
            }),
            a
          );
        var r,
          i = {
            type: e.type,
            content: t(e.content, n),
            tag: "span",
            classes: ["token", e.type],
            attributes: {},
            language: n,
          },
          e = e.alias,
          s =
            (e &&
              (Array.isArray(e)
                ? Array.prototype.push.apply(i.classes, e)
                : i.classes.push(e)),
            F.hooks.run("wrap", i),
            "");
        for (r in i.attributes)
          s +=
            " " +
            r +
            '="' +
            (i.attributes[r] || "").replace(/"/g, "&quot;") +
            '"';
        return (
          "<" +
          i.tag +
          ' class="' +
          i.classes.join(" ") +
          '"' +
          s +
          ">" +
          i.content +
          "</" +
          i.tag +
          ">"
        );
      }),
      l.document
        ? ((a = F.util.currentScript()) &&
            ((F.filename = a.src), a.hasAttribute("data-manual")) &&
            (F.manual = !0),
          F.manual ||
            ("loading" === (e = document.readyState) ||
            ("interactive" === e && a && a.defer)
              ? document.addEventListener("DOMContentLoaded", r)
              : window.requestAnimationFrame
                ? window.requestAnimationFrame(r)
                : window.setTimeout(r, 16)))
        : l.addEventListener &&
          !F.disableWorkerMessageHandler &&
          l.addEventListener(
            "message",
            function (e) {
              var e = JSON.parse(e.data),
                t = e.language,
                n = e.immediateClose;
              (l.postMessage(F.highlight(e.code, F.languages[t], t)),
                n && l.close());
            },
            !1,
          ),
      F
    );
    function r() {
      F.manual || F.highlightAll();
    }
  })(_self);
("undefined" != typeof module && module.exports && (module.exports = Prism),
  "undefined" != typeof global && (global.Prism = Prism),
  (Prism.languages.markup = {
    comment: { pattern: /<!--(?:(?!<!--)[\s\S])*?-->/, greedy: !0 },
    prolog: { pattern: /<\?[\s\S]+?\?>/, greedy: !0 },
    doctype: {
      pattern:
        /<!DOCTYPE(?:[^>"'[\]]|"[^"]*"|'[^']*')+(?:\[(?:[^<"'\]]|"[^"]*"|'[^']*'|<(?!!--)|<!--(?:[^-]|-(?!->))*-->)*\]\s*)?>/i,
      greedy: !0,
      inside: {
        "internal-subset": {
          pattern: /(^[^\[]*\[)[\s\S]+(?=\]>$)/,
          lookbehind: !0,
          greedy: !0,
          inside: null,
        },
        string: { pattern: /"[^"]*"|'[^']*'/, greedy: !0 },
        punctuation: /^<!|>$|[[\]]/,
        "doctype-tag": /^DOCTYPE/i,
        name: /[^\s<>'"]+/,
      },
    },
    cdata: { pattern: /<!\[CDATA\[[\s\S]*?\]\]>/i, greedy: !0 },
    tag: {
      pattern:
        /<\/?(?!\d)[^\s>\/=$<%]+(?:\s(?:\s*[^\s>\/=]+(?:\s*=\s*(?:"[^"]*"|'[^']*'|[^\s'">=]+(?=[\s>]))|(?=[\s/>])))+)?\s*\/?>/,
      greedy: !0,
      inside: {
        tag: {
          pattern: /^<\/?[^\s>\/]+/,
          inside: { punctuation: /^<\/?/, namespace: /^[^\s>\/:]+:/ },
        },
        "special-attr": [],
        "attr-value": {
          pattern: /=\s*(?:"[^"]*"|'[^']*'|[^\s'">=]+)/,
          inside: {
            punctuation: [
              { pattern: /^=/, alias: "attr-equals" },
              { pattern: /^(\s*)["']|["']$/, lookbehind: !0 },
            ],
          },
        },
        punctuation: /\/?>/,
        "attr-name": {
          pattern: /[^\s>\/]+/,
          inside: { namespace: /^[^\s>\/:]+:/ },
        },
      },
    },
    entity: [
      { pattern: /&[\da-z]{1,8};/i, alias: "named-entity" },
      /&#x?[\da-f]{1,8};/i,
    ],
  }),
  (Prism.languages.markup.tag.inside["attr-value"].inside.entity =
    Prism.languages.markup.entity),
  (Prism.languages.markup.doctype.inside["internal-subset"].inside =
    Prism.languages.markup),
  Prism.hooks.add("wrap", function (e) {
    "entity" === e.type &&
      (e.attributes.title = e.content.replace(/&amp;/, "&"));
  }),
  Object.defineProperty(Prism.languages.markup.tag, "addInlined", {
    value: function (e, t) {
      var n = {},
        n =
          ((n["language-" + t] = {
            pattern: /(^<!\[CDATA\[)[\s\S]+?(?=\]\]>$)/i,
            lookbehind: !0,
            inside: Prism.languages[t],
          }),
          (n.cdata = /^<!\[CDATA\[|\]\]>$/i),
          {
            "included-cdata": {
              pattern: /<!\[CDATA\[[\s\S]*?\]\]>/i,
              inside: n,
            },
          }),
        t =
          ((n["language-" + t] = {
            pattern: /[\s\S]+/,
            inside: Prism.languages[t],
          }),
          {});
      ((t[e] = {
        pattern: RegExp(
          /(<__[^>]*>)(?:<!\[CDATA\[(?:[^\]]|\](?!\]>))*\]\]>|(?!<!\[CDATA\[)[\s\S])*?(?=<\/__>)/.source.replace(
            /__/g,
            function () {
              return e;
            },
          ),
          "i",
        ),
        lookbehind: !0,
        greedy: !0,
        inside: n,
      }),
        Prism.languages.insertBefore("markup", "cdata", t));
    },
  }),
  Object.defineProperty(Prism.languages.markup.tag, "addAttribute", {
    value: function (e, t) {
      Prism.languages.markup.tag.inside["special-attr"].push({
        pattern: RegExp(
          /(^|["'\s])/.source +
            "(?:" +
            e +
            ")" +
            /\s*=\s*(?:"[^"]*"|'[^']*'|[^\s'">=]+(?=[\s>]))/.source,
          "i",
        ),
        lookbehind: !0,
        inside: {
          "attr-name": /^[^\s=]+/,
          "attr-value": {
            pattern: /=[\s\S]+/,
            inside: {
              value: {
                pattern: /(^=\s*(["']|(?!["'])))\S[\s\S]*(?=\2$)/,
                lookbehind: !0,
                alias: [t, "language-" + t],
                inside: Prism.languages[t],
              },
              punctuation: [{ pattern: /^=/, alias: "attr-equals" }, /"|'/],
            },
          },
        },
      });
    },
  }),
  (Prism.languages.html = Prism.languages.markup),
  (Prism.languages.mathml = Prism.languages.markup),
  (Prism.languages.svg = Prism.languages.markup),
  (Prism.languages.xml = Prism.languages.extend("markup", {})),
  (Prism.languages.ssml = Prism.languages.xml),
  (Prism.languages.atom = Prism.languages.xml),
  (Prism.languages.rss = Prism.languages.xml),
  ((e) => {
    var t =
      /(?:"(?:\\(?:\r\n|[\s\S])|[^"\\\r\n])*"|'(?:\\(?:\r\n|[\s\S])|[^'\\\r\n])*')/;
    ((e.languages.css = {
      comment: /\/\*[\s\S]*?\*\//,
      atrule: {
        pattern: RegExp(
          "@[\\w-](?:" +
            /[^;{\s"']|\s+(?!\s)/.source +
            "|" +
            t.source +
            ")*?" +
            /(?:;|(?=\s*\{))/.source,
        ),
        inside: {
          rule: /^@[\w-]+/,
          "selector-function-argument": {
            pattern:
              /(\bselector\s*\(\s*(?![\s)]))(?:[^()\s]|\s+(?![\s)])|\((?:[^()]|\([^()]*\))*\))+(?=\s*\))/,
            lookbehind: !0,
            alias: "selector",
          },
          keyword: {
            pattern: /(^|[^\w-])(?:and|not|only|or)(?![\w-])/,
            lookbehind: !0,
          },
        },
      },
      url: {
        pattern: RegExp(
          "\\burl\\((?:" +
            t.source +
            "|" +
            /(?:[^\\\r\n()"']|\\[\s\S])*/.source +
            ")\\)",
          "i",
        ),
        greedy: !0,
        inside: {
          function: /^url/i,
          punctuation: /^\(|\)$/,
          string: { pattern: RegExp("^" + t.source + "$"), alias: "url" },
        },
      },
      selector: {
        pattern: RegExp(
          "(^|[{}\\s])[^{}\\s](?:[^{};\"'\\s]|\\s+(?![\\s{])|" +
            t.source +
            ")*(?=\\s*\\{)",
        ),
        lookbehind: !0,
      },
      string: { pattern: t, greedy: !0 },
      property: {
        pattern:
          /(^|[^-\w\xA0-\uFFFF])(?!\s)[-_a-z\xA0-\uFFFF](?:(?!\s)[-\w\xA0-\uFFFF])*(?=\s*:)/i,
        lookbehind: !0,
      },
      important: /!important\b/i,
      function: { pattern: /(^|[^-a-z0-9])[-a-z0-9]+(?=\()/i, lookbehind: !0 },
      punctuation: /[(){};:,]/,
    }),
      (e.languages.css.atrule.inside.rest = e.languages.css),
      (t = e.languages.markup) &&
        (t.tag.addInlined("style", "css"), t.tag.addAttribute("style", "css")));
  })(Prism),
  (Prism.languages.clike = {
    comment: [
      {
        pattern: /(^|[^\\])\/\*[\s\S]*?(?:\*\/|$)/,
        lookbehind: !0,
        greedy: !0,
      },
      { pattern: /(^|[^\\:])\/\/.*/, lookbehind: !0, greedy: !0 },
    ],
    string: {
      pattern: /(["'])(?:\\(?:\r\n|[\s\S])|(?!\1)[^\\\r\n])*\1/,
      greedy: !0,
    },
    "class-name": {
      pattern:
        /(\b(?:class|extends|implements|instanceof|interface|new|trait)\s+|\bcatch\s+\()[\w.\\]+/i,
      lookbehind: !0,
      inside: { punctuation: /[.\\]/ },
    },
    keyword:
      /\b(?:break|catch|continue|do|else|finally|for|function|if|in|instanceof|new|null|return|throw|try|while)\b/,
    boolean: /\b(?:false|true)\b/,
    function: /\b\w+(?=\()/,
    number: /\b0x[\da-f]+\b|(?:\b\d+(?:\.\d*)?|\B\.\d+)(?:e[+-]?\d+)?/i,
    operator: /[<>]=?|[!=]=?=?|--?|\+\+?|&&?|\|\|?|[?*/~^%]/,
    punctuation: /[{}[\];(),.:]/,
  }),
  (Prism.languages.javascript = Prism.languages.extend("clike", {
    "class-name": [
      Prism.languages.clike["class-name"],
      {
        pattern:
          /(^|[^$\w\xA0-\uFFFF])(?!\s)[_$A-Z\xA0-\uFFFF](?:(?!\s)[$\w\xA0-\uFFFF])*(?=\.(?:constructor|prototype))/,
        lookbehind: !0,
      },
    ],
    keyword: [
      { pattern: /((?:^|\})\s*)catch\b/, lookbehind: !0 },
      {
        pattern:
          /(^|[^.]|\.\.\.\s*)\b(?:as|assert(?=\s*\{)|async(?=\s*(?:function\b|\(|[$\w\xA0-\uFFFF]|$))|await|break|case|class|const|continue|debugger|default|delete|do|else|enum|export|extends|finally(?=\s*(?:\{|$))|for|from(?=\s*(?:['"]|$))|function|(?:get|set)(?=\s*(?:[#\[$\w\xA0-\uFFFF]|$))|if|implements|import|in|instanceof|interface|let|new|null|of|package|private|protected|public|return|static|super|switch|this|throw|try|typeof|undefined|var|void|while|with|yield)\b/,
        lookbehind: !0,
      },
    ],
    function:
      /#?(?!\s)[_$a-zA-Z\xA0-\uFFFF](?:(?!\s)[$\w\xA0-\uFFFF])*(?=\s*(?:\.\s*(?:apply|bind|call)\s*)?\()/,
    number: {
      pattern: RegExp(
        /(^|[^\w$])/.source +
          "(?:" +
          /NaN|Infinity/.source +
          "|" +
          /0[bB][01]+(?:_[01]+)*n?/.source +
          "|" +
          /0[oO][0-7]+(?:_[0-7]+)*n?/.source +
          "|" +
          /0[xX][\dA-Fa-f]+(?:_[\dA-Fa-f]+)*n?/.source +
          "|" +
          /\d+(?:_\d+)*n/.source +
          "|" +
          /(?:\d+(?:_\d+)*(?:\.(?:\d+(?:_\d+)*)?)?|\.\d+(?:_\d+)*)(?:[Ee][+-]?\d+(?:_\d+)*)?/
            .source +
          ")" +
          /(?![\w$])/.source,
      ),
      lookbehind: !0,
    },
    operator:
      /--|\+\+|\*\*=?|=>|&&=?|\|\|=?|[!=]==|<<=?|>>>?=?|[-+*/%&|^!=<>]=?|\.{3}|\?\?=?|\?\.?|[~:]/,
  })),
  (Prism.languages.javascript["class-name"][0].pattern =
    /(\b(?:class|extends|implements|instanceof|interface|new)\s+)[\w.\\]+/),
  Prism.languages.insertBefore("javascript", "keyword", {
    regex: {
      pattern: RegExp(
        /((?:^|[^$\w\xA0-\uFFFF."'\])\s]|\b(?:return|yield))\s*)/.source +
          /\//.source +
          "(?:" +
          /(?:\[(?:[^\]\\\r\n]|\\.)*\]|\\.|[^/\\\[\r\n])+\/[dgimyus]{0,7}/
            .source +
          "|" +
          /(?:\[(?:[^[\]\\\r\n]|\\.|\[(?:[^[\]\\\r\n]|\\.|\[(?:[^[\]\\\r\n]|\\.)*\])*\])*\]|\\.|[^/\\\[\r\n])+\/[dgimyus]{0,7}v[dgimyus]{0,7}/
            .source +
          ")" +
          /(?=(?:\s|\/\*(?:[^*]|\*(?!\/))*\*\/)*(?:$|[\r\n,.;:})\]]|\/\/))/
            .source,
      ),
      lookbehind: !0,
      greedy: !0,
      inside: {
        "regex-source": {
          pattern: /^(\/)[\s\S]+(?=\/[a-z]*$)/,
          lookbehind: !0,
          alias: "language-regex",
          inside: Prism.languages.regex,
        },
        "regex-delimiter": /^\/|\/$/,
        "regex-flags": /^[a-z]+$/,
      },
    },
    "function-variable": {
      pattern:
        /#?(?!\s)[_$a-zA-Z\xA0-\uFFFF](?:(?!\s)[$\w\xA0-\uFFFF])*(?=\s*[=:]\s*(?:async\s*)?(?:\bfunction\b|(?:\((?:[^()]|\([^()]*\))*\)|(?!\s)[_$a-zA-Z\xA0-\uFFFF](?:(?!\s)[$\w\xA0-\uFFFF])*)\s*=>))/,
      alias: "function",
    },
    parameter: [
      {
        pattern:
          /(function(?:\s+(?!\s)[_$a-zA-Z\xA0-\uFFFF](?:(?!\s)[$\w\xA0-\uFFFF])*)?\s*\(\s*)(?!\s)(?:[^()\s]|\s+(?![\s)])|\([^()]*\))+(?=\s*\))/,
        lookbehind: !0,
        inside: Prism.languages.javascript,
      },
      {
        pattern:
          /(^|[^$\w\xA0-\uFFFF])(?!\s)[_$a-z\xA0-\uFFFF](?:(?!\s)[$\w\xA0-\uFFFF])*(?=\s*=>)/i,
        lookbehind: !0,
        inside: Prism.languages.javascript,
      },
      {
        pattern:
          /(\(\s*)(?!\s)(?:[^()\s]|\s+(?![\s)])|\([^()]*\))+(?=\s*\)\s*=>)/,
        lookbehind: !0,
        inside: Prism.languages.javascript,
      },
      {
        pattern:
          /((?:\b|\s|^)(?!(?:as|async|await|break|case|catch|class|const|continue|debugger|default|delete|do|else|enum|export|extends|finally|for|from|function|get|if|implements|import|in|instanceof|interface|let|new|null|of|package|private|protected|public|return|set|static|super|switch|this|throw|try|typeof|undefined|var|void|while|with|yield)(?![$\w\xA0-\uFFFF]))(?:(?!\s)[_$a-zA-Z\xA0-\uFFFF](?:(?!\s)[$\w\xA0-\uFFFF])*\s*)\(\s*|\]\s*\(\s*)(?!\s)(?:[^()\s]|\s+(?![\s)])|\([^()]*\))+(?=\s*\)\s*\{)/,
        lookbehind: !0,
        inside: Prism.languages.javascript,
      },
    ],
    constant: /\b[A-Z](?:[A-Z_]|\dx?)*\b/,
  }),
  Prism.languages.insertBefore("javascript", "string", {
    hashbang: { pattern: /^#!.*/, greedy: !0, alias: "comment" },
    "template-string": {
      pattern:
        /`(?:\\[\s\S]|\$\{(?:[^{}]|\{(?:[^{}]|\{[^}]*\})*\})+\}|(?!\$\{)[^\\`])*`/,
      greedy: !0,
      inside: {
        "template-punctuation": { pattern: /^`|`$/, alias: "string" },
        interpolation: {
          pattern:
            /((?:^|[^\\])(?:\\{2})*)\$\{(?:[^{}]|\{(?:[^{}]|\{[^}]*\})*\})+\}/,
          lookbehind: !0,
          inside: {
            "interpolation-punctuation": {
              pattern: /^\$\{|\}$/,
              alias: "punctuation",
            },
            rest: Prism.languages.javascript,
          },
        },
        string: /[\s\S]+/,
      },
    },
    "string-property": {
      pattern:
        /((?:^|[,{])[ \t]*)(["'])(?:\\(?:\r\n|[\s\S])|(?!\2)[^\\\r\n])*\2(?=\s*:)/m,
      lookbehind: !0,
      greedy: !0,
      alias: "property",
    },
  }),
  Prism.languages.insertBefore("javascript", "operator", {
    "literal-property": {
      pattern:
        /((?:^|[,{])[ \t]*)(?!\s)[_$a-zA-Z\xA0-\uFFFF](?:(?!\s)[$\w\xA0-\uFFFF])*(?=\s*:)/m,
      lookbehind: !0,
      alias: "property",
    },
  }),
  Prism.languages.markup &&
    (Prism.languages.markup.tag.addInlined("script", "javascript"),
    Prism.languages.markup.tag.addAttribute(
      /on(?:abort|blur|change|click|composition(?:end|start|update)|dblclick|error|focus(?:in|out)?|key(?:down|up)|load|mouse(?:down|enter|leave|move|out|over|up)|reset|resize|scroll|select|slotchange|submit|unload|wheel)/
        .source,
      "javascript",
    )),
  (Prism.languages.js = Prism.languages.javascript),
  ((e) => {
    for (
      var t =
          "\\b(?:BASH|BASHOPTS|BASH_ALIASES|BASH_ARGC|BASH_ARGV|BASH_CMDS|BASH_COMPLETION_COMPAT_DIR|BASH_LINENO|BASH_REMATCH|BASH_SOURCE|BASH_VERSINFO|BASH_VERSION|COLORTERM|COLUMNS|COMP_WORDBREAKS|DBUS_SESSION_BUS_ADDRESS|DEFAULTS_PATH|DESKTOP_SESSION|DIRSTACK|DISPLAY|EUID|GDMSESSION|GDM_LANG|GNOME_KEYRING_CONTROL|GNOME_KEYRING_PID|GPG_AGENT_INFO|GROUPS|HISTCONTROL|HISTFILE|HISTFILESIZE|HISTSIZE|HOME|HOSTNAME|HOSTTYPE|IFS|INSTANCE|JOB|LANG|LANGUAGE|LC_ADDRESS|LC_ALL|LC_IDENTIFICATION|LC_MEASUREMENT|LC_MONETARY|LC_NAME|LC_NUMERIC|LC_PAPER|LC_TELEPHONE|LC_TIME|LESSCLOSE|LESSOPEN|LINES|LOGNAME|LS_COLORS|MACHTYPE|MAILCHECK|MANDATORY_PATH|NO_AT_BRIDGE|OLDPWD|OPTERR|OPTIND|ORBIT_SOCKETDIR|OSTYPE|PAPERSIZE|PATH|PIPESTATUS|PPID|PS1|PS2|PS3|PS4|PWD|RANDOM|REPLY|SECONDS|SELINUX_INIT|SESSION|SESSIONTYPE|SESSION_MANAGER|SHELL|SHELLOPTS|SHLVL|SSH_AUTH_SOCK|TERM|UID|UPSTART_EVENTS|UPSTART_INSTANCE|UPSTART_JOB|UPSTART_SESSION|USER|WINDOWID|XAUTHORITY|XDG_CONFIG_DIRS|XDG_CURRENT_DESKTOP|XDG_DATA_DIRS|XDG_GREETER_DATA_DIR|XDG_MENU_PREFIX|XDG_RUNTIME_DIR|XDG_SEAT|XDG_SEAT_PATH|XDG_SESSION_DESKTOP|XDG_SESSION_ID|XDG_SESSION_PATH|XDG_SESSION_TYPE|XDG_VTNR|XMODIFIERS)\\b",
        n = {
          pattern: /(^(["']?)\w+\2)[ \t]+\S.*/,
          lookbehind: !0,
          alias: "punctuation",
          inside: null,
        },
        a = {
          bash: n,
          environment: { pattern: RegExp("\\$" + t), alias: "constant" },
          variable: [
            {
              pattern: /\$?\(\([\s\S]+?\)\)/,
              greedy: !0,
              inside: {
                variable: [
                  { pattern: /(^\$\(\([\s\S]+)\)\)/, lookbehind: !0 },
                  /^\$\(\(/,
                ],
                number:
                  /\b0x[\dA-Fa-f]+\b|(?:\b\d+(?:\.\d*)?|\B\.\d+)(?:[Ee]-?\d+)?/,
                operator:
                  /--|\+\+|\*\*=?|<<=?|>>=?|&&|\|\||[=!+\-*/%<>^&|]=?|[?~:]/,
                punctuation: /\(\(?|\)\)?|,|;/,
              },
            },
            {
              pattern: /\$\((?:\([^)]+\)|[^()])+\)|`[^`]+`/,
              greedy: !0,
              inside: { variable: /^\$\(|^`|\)$|`$/ },
            },
            {
              pattern: /\$\{[^}]+\}/,
              greedy: !0,
              inside: {
                operator: /:[-=?+]?|[!\/]|##?|%%?|\^\^?|,,?/,
                punctuation: /[\[\]]/,
                environment: {
                  pattern: RegExp("(\\{)" + t),
                  lookbehind: !0,
                  alias: "constant",
                },
              },
            },
            /\$(?:\w+|[#?*!@$])/,
          ],
          entity:
            /\\(?:[abceEfnrtv\\"]|O?[0-7]{1,3}|U[0-9a-fA-F]{8}|u[0-9a-fA-F]{4}|x[0-9a-fA-F]{1,2})/,
        },
        r =
          ((e.languages.bash = {
            shebang: { pattern: /^#!\s*\/.*/, alias: "important" },
            comment: { pattern: /(^|[^"{\\$])#.*/, lookbehind: !0 },
            "function-name": [
              {
                pattern: /(\bfunction\s+)[\w-]+(?=(?:\s*\(?:\s*\))?\s*\{)/,
                lookbehind: !0,
                alias: "function",
              },
              { pattern: /\b[\w-]+(?=\s*\(\s*\)\s*\{)/, alias: "function" },
            ],
            "for-or-select": {
              pattern: /(\b(?:for|select)\s+)\w+(?=\s+in\s)/,
              alias: "variable",
              lookbehind: !0,
            },
            "assign-left": {
              pattern: /(^|[\s;|&]|[<>]\()\w+(?:\.\w+)*(?=\+?=)/,
              inside: {
                environment: {
                  pattern: RegExp("(^|[\\s;|&]|[<>]\\()" + t),
                  lookbehind: !0,
                  alias: "constant",
                },
              },
              alias: "variable",
              lookbehind: !0,
            },
            parameter: {
              pattern: /(^|\s)-{1,2}(?:\w+:[+-]?)?\w+(?:\.\w+)*(?=[=\s]|$)/,
              alias: "variable",
              lookbehind: !0,
            },
            string: [
              {
                pattern: /((?:^|[^<])<<-?\s*)(\w+)\s[\s\S]*?(?:\r?\n|\r)\2/,
                lookbehind: !0,
                greedy: !0,
                inside: a,
              },
              {
                pattern:
                  /((?:^|[^<])<<-?\s*)(["'])(\w+)\2\s[\s\S]*?(?:\r?\n|\r)\3/,
                lookbehind: !0,
                greedy: !0,
                inside: { bash: n },
              },
              {
                pattern:
                  /(^|[^\\](?:\\\\)*)"(?:\\[\s\S]|\$\([^)]+\)|\$(?!\()|`[^`]+`|[^"\\`$])*"/,
                lookbehind: !0,
                greedy: !0,
                inside: a,
              },
              { pattern: /(^|[^$\\])'[^']*'/, lookbehind: !0, greedy: !0 },
              {
                pattern: /\$'(?:[^'\\]|\\[\s\S])*'/,
                greedy: !0,
                inside: { entity: a.entity },
              },
            ],
            environment: { pattern: RegExp("\\$?" + t), alias: "constant" },
            variable: a.variable,
            function: {
              pattern:
                /(^|[\s;|&]|[<>]\()(?:add|apropos|apt|apt-cache|apt-get|aptitude|aspell|automysqlbackup|awk|basename|bash|bc|bconsole|bg|bzip2|cal|cargo|cat|cfdisk|chgrp|chkconfig|chmod|chown|chroot|cksum|clear|cmp|column|comm|composer|cp|cron|crontab|csplit|curl|cut|date|dc|dd|ddrescue|debootstrap|df|diff|diff3|dig|dir|dircolors|dirname|dirs|dmesg|docker|docker-compose|du|egrep|eject|env|ethtool|expand|expect|expr|fdformat|fdisk|fg|fgrep|file|find|fmt|fold|format|free|fsck|ftp|fuser|gawk|git|gparted|grep|groupadd|groupdel|groupmod|groups|grub-mkconfig|gzip|halt|head|hg|history|host|hostname|htop|iconv|id|ifconfig|ifdown|ifup|import|install|ip|java|jobs|join|kill|killall|less|link|ln|locate|logname|logrotate|look|lpc|lpr|lprint|lprintd|lprintq|lprm|ls|lsof|lynx|make|man|mc|mdadm|mkconfig|mkdir|mke2fs|mkfifo|mkfs|mkisofs|mknod|mkswap|mmv|more|most|mount|mtools|mtr|mutt|mv|nano|nc|netstat|nice|nl|node|nohup|notify-send|npm|nslookup|op|open|parted|passwd|paste|pathchk|ping|pkill|pnpm|podman|podman-compose|popd|pr|printcap|printenv|ps|pushd|pv|quota|quotacheck|quotactl|ram|rar|rcp|reboot|remsync|rename|renice|rev|rm|rmdir|rpm|rsync|scp|screen|sdiff|sed|sendmail|seq|service|sftp|sh|shellcheck|shuf|shutdown|sleep|slocate|sort|split|ssh|stat|strace|su|sudo|sum|suspend|swapon|sync|sysctl|tac|tail|tar|tee|time|timeout|top|touch|tr|traceroute|tsort|tty|umount|uname|unexpand|uniq|units|unrar|unshar|unzip|update-grub|uptime|useradd|userdel|usermod|users|uudecode|uuencode|v|vcpkg|vdir|vi|vim|virsh|vmstat|wait|watch|wc|wget|whereis|which|who|whoami|write|xargs|xdg-open|yarn|yes|zenity|zip|zsh|zypper)(?=$|[)\s;|&])/,
              lookbehind: !0,
            },
            keyword: {
              pattern:
                /(^|[\s;|&]|[<>]\()(?:case|do|done|elif|else|esac|fi|for|function|if|in|select|then|until|while)(?=$|[)\s;|&])/,
              lookbehind: !0,
            },
            builtin: {
              pattern:
                /(^|[\s;|&]|[<>]\()(?:\.|:|alias|bind|break|builtin|caller|cd|command|continue|declare|echo|enable|eval|exec|exit|export|getopts|hash|help|let|local|logout|mapfile|printf|pwd|read|readarray|readonly|return|set|shift|shopt|source|test|times|trap|type|typeset|ulimit|umask|unalias|unset)(?=$|[)\s;|&])/,
              lookbehind: !0,
              alias: "class-name",
            },
            boolean: {
              pattern: /(^|[\s;|&]|[<>]\()(?:false|true)(?=$|[)\s;|&])/,
              lookbehind: !0,
            },
            "file-descriptor": { pattern: /\B&\d\b/, alias: "important" },
            operator: {
              pattern:
                /\d?<>|>\||\+=|=[=~]?|!=?|<<[<-]?|[&\d]?>>|\d[<>]&?|[<>][&=]?|&[>&]?|\|[&|]?/,
              inside: {
                "file-descriptor": { pattern: /^\d/, alias: "important" },
              },
            },
            punctuation: /\$?\(\(?|\)\)?|\.\.|[{}[\];\\]/,
            number: {
              pattern: /(^|\s)(?:[1-9]\d*|0)(?:[.,]\d+)?\b/,
              lookbehind: !0,
            },
          }),
          (n.inside = e.languages.bash),
          [
            "comment",
            "function-name",
            "for-or-select",
            "assign-left",
            "parameter",
            "string",
            "environment",
            "function",
            "keyword",
            "builtin",
            "boolean",
            "file-descriptor",
            "operator",
            "punctuation",
            "number",
          ]),
        i = a.variable[1].inside,
        s = 0;
      s < r.length;
      s++
    )
      i[r[s]] = e.languages.bash[r[s]];
    ((e.languages.sh = e.languages.bash),
      (e.languages.shell = e.languages.bash));
  })(Prism),
  (Prism.languages.json = {
    property: {
      pattern: /(^|[^\\])"(?:\\.|[^\\"\r\n])*"(?=\s*:)/,
      lookbehind: !0,
      greedy: !0,
    },
    string: {
      pattern: /(^|[^\\])"(?:\\.|[^\\"\r\n])*"(?!\s*:)/,
      lookbehind: !0,
      greedy: !0,
    },
    comment: { pattern: /\/\/.*|\/\*[\s\S]*?(?:\*\/|$)/, greedy: !0 },
    number: /-?\b\d+(?:\.\d+)?(?:e[+-]?\d+)?\b/i,
    punctuation: /[{}[\],]/,
    operator: /:/,
    boolean: /\b(?:false|true)\b/,
    null: { pattern: /\bnull\b/, alias: "keyword" },
  }),
  (Prism.languages.webmanifest = Prism.languages.json),
  (Prism.languages.python = {
    comment: { pattern: /(^|[^\\])#.*/, lookbehind: !0, greedy: !0 },
    "string-interpolation": {
      pattern:
        /(?:f|fr|rf)(?:("""|''')[\s\S]*?\1|("|')(?:\\.|(?!\2)[^\\\r\n])*\2)/i,
      greedy: !0,
      inside: {
        interpolation: {
          pattern:
            /((?:^|[^{])(?:\{\{)*)\{(?!\{)(?:[^{}]|\{(?!\{)(?:[^{}]|\{(?!\{)(?:[^{}])+\})+\})+\}/,
          lookbehind: !0,
          inside: {
            "format-spec": { pattern: /(:)[^:(){}]+(?=\}$)/, lookbehind: !0 },
            "conversion-option": {
              pattern: /![sra](?=[:}]$)/,
              alias: "punctuation",
            },
            rest: null,
          },
        },
        string: /[\s\S]+/,
      },
    },
    "triple-quoted-string": {
      pattern: /(?:[rub]|br|rb)?("""|''')[\s\S]*?\1/i,
      greedy: !0,
      alias: "string",
    },
    string: {
      pattern: /(?:[rub]|br|rb)?("|')(?:\\.|(?!\1)[^\\\r\n])*\1/i,
      greedy: !0,
    },
    function: {
      pattern: /((?:^|\s)def[ \t]+)[a-zA-Z_]\w*(?=\s*\()/g,
      lookbehind: !0,
    },
    "class-name": { pattern: /(\bclass\s+)\w+/i, lookbehind: !0 },
    decorator: {
      pattern: /(^[\t ]*)@\w+(?:\.\w+)*/m,
      lookbehind: !0,
      alias: ["annotation", "punctuation"],
      inside: { punctuation: /\./ },
    },
    keyword:
      /\b(?:_(?=\s*:)|and|as|assert|async|await|break|case|class|continue|def|del|elif|else|except|exec|finally|for|from|global|if|import|in|is|lambda|match|nonlocal|not|or|pass|print|raise|return|try|while|with|yield)\b/,
    builtin:
      /\b(?:__import__|abs|all|any|apply|ascii|basestring|bin|bool|buffer|bytearray|bytes|callable|chr|classmethod|cmp|coerce|compile|complex|delattr|dict|dir|divmod|enumerate|eval|execfile|file|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|intern|isinstance|issubclass|iter|len|list|locals|long|map|max|memoryview|min|next|object|oct|open|ord|pow|property|range|raw_input|reduce|reload|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|unichr|unicode|vars|xrange|zip)\b/,
    boolean: /\b(?:False|None|True)\b/,
    number:
      /\b0(?:b(?:_?[01])+|o(?:_?[0-7])+|x(?:_?[a-f0-9])+)\b|(?:\b\d+(?:_\d+)*(?:\.(?:\d+(?:_\d+)*)?)?|\B\.\d+(?:_\d+)*)(?:e[+-]?\d+(?:_\d+)*)?j?(?!\w)/i,
    operator: /[-+%=]=?|!=|:=|\*\*?=?|\/\/?=?|<[<=>]?|>[=>]?|[&|^~]/,
    punctuation: /[{}[\];(),.:]/,
  }),
  (Prism.languages.python[
    "string-interpolation"
  ].inside.interpolation.inside.rest = Prism.languages.python),
  (Prism.languages.py = Prism.languages.python),
  ((i) => {
    var e = i.util.clone(i.languages.javascript),
      n = /(?:\s|\/\/.*(?!.)|\/\*(?:[^*]|\*(?!\/))\*\/)/.source,
      a = /(?:\{(?:\{(?:\{[^{}]*\}|[^{}])*\}|[^{}])*\})/.source,
      r = /(?:\{<S>*\.{3}(?:[^{}]|<BRACES>)*\})/.source;
    function t(e, t) {
      return (
        (e = e
          .replace(/<S>/g, function () {
            return n;
          })
          .replace(/<BRACES>/g, function () {
            return a;
          })
          .replace(/<SPREAD>/g, function () {
            return r;
          })),
        RegExp(e, t)
      );
    }
    function s(e) {
      for (var t = [], n = 0; n < e.length; n++) {
        var a = e[n],
          r = !1;
        ("string" != typeof a &&
          ("tag" === a.type && a.content[0] && "tag" === a.content[0].type
            ? "</" === a.content[0].content[0].content
              ? 0 < t.length &&
                t[t.length - 1].tagName === o(a.content[0].content[1]) &&
                t.pop()
              : "/>" !== a.content[a.content.length - 1].content &&
                t.push({ tagName: o(a.content[0].content[1]), openedBraces: 0 })
            : 0 < t.length && "punctuation" === a.type && "{" === a.content
              ? t[t.length - 1].openedBraces++
              : 0 < t.length &&
                  0 < t[t.length - 1].openedBraces &&
                  "punctuation" === a.type &&
                  "}" === a.content
                ? t[t.length - 1].openedBraces--
                : (r = !0)),
          (r || "string" == typeof a) &&
            0 < t.length &&
            0 === t[t.length - 1].openedBraces &&
            ((r = o(a)),
            n < e.length - 1 &&
              ("string" == typeof e[n + 1] || "plain-text" === e[n + 1].type) &&
              ((r += o(e[n + 1])), e.splice(n + 1, 1)),
            0 < n &&
              ("string" == typeof e[n - 1] || "plain-text" === e[n - 1].type) &&
              ((r = o(e[n - 1]) + r), e.splice(n - 1, 1), n--),
            (e[n] = new i.Token("plain-text", r, null, r))),
          a.content && "string" != typeof a.content && s(a.content));
      }
    }
    ((r = t(r).source),
      (i.languages.jsx = i.languages.extend("markup", e)),
      (i.languages.jsx.tag.pattern = t(
        /<\/?(?:[\w.:-]+(?:<S>+(?:[\w.:$-]+(?:=(?:"(?:\\[\s\S]|[^\\"])*"|'(?:\\[\s\S]|[^\\'])*'|[^\s{'"/>=]+|<BRACES>))?|<SPREAD>))*<S>*\/?)?>/
          .source,
      )),
      (i.languages.jsx.tag.inside.tag.pattern = /^<\/?[^\s>\/]*/),
      (i.languages.jsx.tag.inside["attr-value"].pattern =
        /=(?!\{)(?:"(?:\\[\s\S]|[^\\"])*"|'(?:\\[\s\S]|[^\\'])*'|[^\s'">]+)/),
      (i.languages.jsx.tag.inside.tag.inside["class-name"] =
        /^[A-Z]\w*(?:\.[A-Z]\w*)*$/),
      (i.languages.jsx.tag.inside.comment = e.comment),
      i.languages.insertBefore(
        "inside",
        "attr-name",
        { spread: { pattern: t(/<SPREAD>/.source), inside: i.languages.jsx } },
        i.languages.jsx.tag,
      ),
      i.languages.insertBefore(
        "inside",
        "special-attr",
        {
          script: {
            pattern: t(/=<BRACES>/.source),
            alias: "language-javascript",
            inside: {
              "script-punctuation": {
                pattern: /^=(?=\{)/,
                alias: "punctuation",
              },
              rest: i.languages.jsx,
            },
          },
        },
        i.languages.jsx.tag,
      ));
    var o = function (e) {
      return e
        ? "string" == typeof e
          ? e
          : "string" == typeof e.content
            ? e.content
            : e.content.map(o).join("")
        : "";
    };
    i.hooks.add("after-tokenize", function (e) {
      ("jsx" !== e.language && "tsx" !== e.language) || s(e.tokens);
    });
  })(Prism),
  (Prism.languages.swift = {
    comment: {
      pattern:
        /(^|[^\\:])(?:\/\/.*|\/\*(?:[^/*]|\/(?!\*)|\*(?!\/)|\/\*(?:[^*]|\*(?!\/))*\*\/)*\*\/)/,
      lookbehind: !0,
      greedy: !0,
    },
    "string-literal": [
      {
        pattern: RegExp(
          /(^|[^"#])/.source +
            "(?:" +
            /"(?:\\(?:\((?:[^()]|\([^()]*\))*\)|\r\n|[^(])|[^\\\r\n"])*"/
              .source +
            "|" +
            /"""(?:\\(?:\((?:[^()]|\([^()]*\))*\)|[^(])|[^\\"]|"(?!""))*"""/
              .source +
            ")" +
            /(?!["#])/.source,
        ),
        lookbehind: !0,
        greedy: !0,
        inside: {
          interpolation: {
            pattern: /(\\\()(?:[^()]|\([^()]*\))*(?=\))/,
            lookbehind: !0,
            inside: null,
          },
          "interpolation-punctuation": {
            pattern: /^\)|\\\($/,
            alias: "punctuation",
          },
          punctuation: /\\(?=[\r\n])/,
          string: /[\s\S]+/,
        },
      },
      {
        pattern: RegExp(
          /(^|[^"#])(#+)/.source +
            "(?:" +
            /"(?:\\(?:#+\((?:[^()]|\([^()]*\))*\)|\r\n|[^#])|[^\\\r\n])*?"/
              .source +
            "|" +
            /"""(?:\\(?:#+\((?:[^()]|\([^()]*\))*\)|[^#])|[^\\])*?"""/.source +
            ")\\2",
        ),
        lookbehind: !0,
        greedy: !0,
        inside: {
          interpolation: {
            pattern: /(\\#+\()(?:[^()]|\([^()]*\))*(?=\))/,
            lookbehind: !0,
            inside: null,
          },
          "interpolation-punctuation": {
            pattern: /^\)|\\#+\($/,
            alias: "punctuation",
          },
          string: /[\s\S]+/,
        },
      },
    ],
    directive: {
      pattern: RegExp(
        /#/.source +
          "(?:" +
          /(?:elseif|if)\b/.source +
          "(?:[ \t]*" +
          /(?:![ \t]*)?(?:\b\w+\b(?:[ \t]*\((?:[^()]|\([^()]*\))*\))?|\((?:[^()]|\([^()]*\))*\))(?:[ \t]*(?:&&|\|\|))?/
            .source +
          ")+|" +
          /(?:else|endif)\b/.source +
          ")",
      ),
      alias: "property",
      inside: {
        "directive-name": /^#\w+/,
        boolean: /\b(?:false|true)\b/,
        number: /\b\d+(?:\.\d+)*\b/,
        operator: /!|&&|\|\||[<>]=?/,
        punctuation: /[(),]/,
      },
    },
    literal: {
      pattern:
        /#(?:colorLiteral|column|dsohandle|file(?:ID|Literal|Path)?|function|imageLiteral|line)\b/,
      alias: "constant",
    },
    "other-directive": { pattern: /#\w+\b/, alias: "property" },
    attribute: { pattern: /@\w+/, alias: "atrule" },
    "function-definition": {
      pattern: /(\bfunc\s+)\w+/,
      lookbehind: !0,
      alias: "function",
    },
    label: {
      pattern:
        /\b(break|continue)\s+\w+|\b[a-zA-Z_]\w*(?=\s*:\s*(?:for|repeat|while)\b)/,
      lookbehind: !0,
      alias: "important",
    },
    keyword:
      /\b(?:Any|Protocol|Self|Type|actor|as|assignment|associatedtype|associativity|async|await|break|case|catch|class|continue|convenience|default|defer|deinit|didSet|do|dynamic|else|enum|extension|fallthrough|fileprivate|final|for|func|get|guard|higherThan|if|import|in|indirect|infix|init|inout|internal|is|isolated|lazy|left|let|lowerThan|mutating|none|nonisolated|nonmutating|open|operator|optional|override|postfix|precedencegroup|prefix|private|protocol|public|repeat|required|rethrows|return|right|safe|self|set|some|static|struct|subscript|super|switch|throw|throws|try|typealias|unowned|unsafe|var|weak|where|while|willSet)\b/,
    boolean: /\b(?:false|true)\b/,
    nil: { pattern: /\bnil\b/, alias: "constant" },
    "short-argument": /\$\d+\b/,
    omit: { pattern: /\b_\b/, alias: "keyword" },
    number:
      /\b(?:[\d_]+(?:\.[\de_]+)?|0x[a-f0-9_]+(?:\.[a-f0-9p_]+)?|0b[01_]+|0o[0-7_]+)\b/i,
    "class-name": /\b[A-Z](?:[A-Z_\d]*[a-z]\w*)?\b/,
    function: /\b[a-z_]\w*(?=\s*\()/i,
    constant: /\b(?:[A-Z_]{2,}|k[A-Z][A-Za-z_]+)\b/,
    operator: /[-+*/%=!<>&|^~?]+|\.[.\-+*/%=!<>&|^~?]+/,
    punctuation: /[{}[\]();,.:\\]/,
  }),
  Prism.languages.swift["string-literal"].forEach(function (e) {
    e.inside.interpolation.inside = Prism.languages.swift;
  }),
  (() => {
    var o, b, y, S, t, k, r;
    function A(e, t) {
      return Array.prototype.slice.call((t || document).querySelectorAll(e));
    }
    function v(e, t) {
      return e.classList.contains(t);
    }
    function w(e) {
      e();
    }
    function i(e) {
      return !!(
        e &&
        /pre/i.test(e.nodeName) &&
        (e.hasAttribute("data-line") || (e.id && Prism.util.isActive(e, b)))
      );
    }
    function s() {
      var e = location.hash.slice(1),
        t =
          (A(".temporary.line-highlight").forEach(function (e) {
            e.parentNode.removeChild(e);
          }),
          (e.match(/\.([\d,-]+)$/) || [, ""])[1]);
      t &&
        !document.getElementById(e) &&
        ((e = e.slice(0, e.lastIndexOf("."))),
        (e = document.getElementById(e))) &&
        (e.hasAttribute("data-line") || e.setAttribute("data-line", ""),
        Prism.plugins.lineHighlight.highlightLines(e, t, "temporary ")(),
        k) &&
        document.querySelector(".temporary.line-highlight").scrollIntoView();
    }
    void 0 !== Prism &&
      "undefined" != typeof document &&
      document.querySelector &&
      ((o = "line-numbers"),
      (b = "linkable-line-numbers"),
      (y = /\n(?!$)/g),
      (S = function () {
        var e;
        return (
          void 0 === t &&
            (((e = document.createElement("div")).style.fontSize = "13px"),
            (e.style.lineHeight = "1.5"),
            (e.style.padding = "0"),
            (e.style.border = "0"),
            (e.innerHTML = "&nbsp;<br />&nbsp;"),
            document.body.appendChild(e),
            (t = 38 === e.offsetHeight),
            document.body.removeChild(e)),
          t
        );
      }),
      (k = !0),
      (Prism.plugins.lineHighlight = {
        highlightLines: function (l, e, u) {
          var t,
            e = (e =
              "string" == typeof e ? e : l.getAttribute("data-line") || "")
              .replace(/\s+/g, "")
              .split(",")
              .filter(Boolean),
            c = +l.getAttribute("data-line-offset") || 0,
            p = (S() ? parseInt : parseFloat)(getComputedStyle(l).lineHeight),
            d = Prism.util.isActive(l, o),
            n = l.querySelector("code"),
            g = (!d && n) || l,
            m = [],
            a = n.textContent.match(y),
            h = a ? a.length + 1 : 1,
            f =
              n && g != n
                ? ((a = l),
                  (n = n),
                  (a = getComputedStyle(l)),
                  (t = getComputedStyle(n)),
                  n.offsetTop +
                    r(t.borderTopWidth) +
                    r(t.paddingTop) -
                    r(a.paddingTop))
                : 0;
          function r(e) {
            return +e.substr(0, e.length - 2);
          }
          e.forEach(function (e) {
            var t,
              n,
              a,
              r,
              i = e.split("-"),
              s = +i[0],
              o = +i[1] || s;
            (o = Math.min(h + c, o)) < s ||
              ((t =
                l.querySelector('.line-highlight[data-range="' + e + '"]') ||
                document.createElement("div")),
              m.push(function () {
                (t.setAttribute("aria-hidden", "true"),
                  t.setAttribute("data-range", e),
                  (t.className = (u || "") + " line-highlight"));
              }),
              d && Prism.plugins.lineNumbers
                ? ((i = Prism.plugins.lineNumbers.getLine(l, s)),
                  (n = Prism.plugins.lineNumbers.getLine(l, o)),
                  i &&
                    ((a = i.offsetTop + f + "px"),
                    m.push(function () {
                      t.style.top = a;
                    })),
                  n &&
                    ((r = n.offsetTop - i.offsetTop + n.offsetHeight + "px"),
                    m.push(function () {
                      t.style.height = r;
                    })))
                : m.push(function () {
                    (t.setAttribute("data-start", String(s)),
                      s < o && t.setAttribute("data-end", String(o)),
                      (t.style.top = (s - c - 1) * p + f + "px"),
                      (t.textContent = new Array(o - s + 2).join(" \n")));
                  }),
              m.push(function () {
                t.style.width = l.scrollWidth + "px";
              }),
              m.push(function () {
                g.appendChild(t);
              }));
          });
          var i,
            s = l.id;
          return (
            d &&
              Prism.util.isActive(l, b) &&
              s &&
              (v(l, b) ||
                m.push(function () {
                  l.classList.add(b);
                }),
              (i = parseInt(l.getAttribute("data-start") || "1")),
              A(".line-numbers-rows > span", l).forEach(function (e, t) {
                var n = t + i;
                e.onclick = function () {
                  ((k = !1),
                    (location.hash = s + "." + n),
                    setTimeout(function () {
                      k = !0;
                    }, 1));
                };
              })),
            function () {
              m.forEach(w);
            }
          );
        },
      }),
      (r = 0),
      Prism.hooks.add("before-sanity-check", function (e) {
        var t,
          n = e.element.parentElement;
        i(n) &&
          ((t = 0),
          A(".line-highlight", n).forEach(function (e) {
            ((t += e.textContent.length), e.parentNode.removeChild(e));
          }),
          t) &&
          /^(?: \n)+$/.test(e.code.slice(-t)) &&
          (e.code = e.code.slice(0, -t));
      }),
      Prism.hooks.add("complete", function e(t) {
        var n,
          a = t.element.parentElement;
        i(a) &&
          (clearTimeout(r),
          (n = Prism.plugins.lineNumbers),
          (t = t.plugins && t.plugins.lineNumbers),
          v(a, o) && n && !t
            ? Prism.hooks.add("line-numbers", e)
            : (Prism.plugins.lineHighlight.highlightLines(a)(),
              (r = setTimeout(s, 1))));
      }),
      window.addEventListener("hashchange", s),
      window.addEventListener("resize", function () {
        A("pre")
          .filter(i)
          .map(function (e) {
            return Prism.plugins.lineHighlight.highlightLines(e);
          })
          .forEach(w);
      }));
  })(),
  (() => {
    var r, i, e, t;
    function s(e) {
      0 !=
        (e = e.filter(function (e) {
          e = (
            (e = e)
              ? window.getComputedStyle
                ? getComputedStyle(e)
                : e.currentStyle || null
              : null
          )["white-space"];
          return "pre-wrap" === e || "pre-line" === e;
        })).length &&
        ((e = e
          .map(function (e) {
            var t,
              n = e.querySelector("code"),
              a = e.querySelector(".line-numbers-rows");
            if (n && a)
              return (
                (a = e.querySelector(".line-numbers-sizer")),
                (t = n.textContent.split(i)),
                a ||
                  (((a = document.createElement("span")).className =
                    "line-numbers-sizer"),
                  n.appendChild(a)),
                (a.innerHTML = "0"),
                (a.style.display = "block"),
                (n = a.getBoundingClientRect().height),
                (a.innerHTML = ""),
                {
                  element: e,
                  lines: t,
                  lineHeights: [],
                  oneLinerHeight: n,
                  sizer: a,
                }
              );
          })
          .filter(Boolean)).forEach(function (e) {
          var a = e.sizer,
            t = e.lines,
            r = e.lineHeights,
            i = e.oneLinerHeight;
          ((r[t.length - 1] = void 0),
            t.forEach(function (e, t) {
              var n;
              e && 1 < e.length
                ? (((n = a.appendChild(
                    document.createElement("span"),
                  )).style.display = "block"),
                  (n.textContent = e))
                : (r[t] = i);
            }));
        }),
        e.forEach(function (e) {
          for (
            var t = e.sizer, n = e.lineHeights, a = 0, r = 0;
            r < n.length;
            r++
          )
            void 0 === n[r] &&
              (n[r] = t.children[a++].getBoundingClientRect().height);
        }),
        e.forEach(function (e) {
          var t = e.sizer,
            n = e.element.querySelector(".line-numbers-rows");
          ((t.style.display = "none"),
            (t.innerHTML = ""),
            e.lineHeights.forEach(function (e, t) {
              n.children[t].style.height = e + "px";
            }));
        }));
    }
    void 0 !== Prism &&
      "undefined" != typeof document &&
      ((r = "line-numbers"),
      (i = /\n(?!$)/g),
      (e = Prism.plugins.lineNumbers =
        {
          getLine: function (e, t) {
            if ("PRE" === e.tagName && e.classList.contains(r)) {
              var n,
                a = e.querySelector(".line-numbers-rows");
              if (a)
                return (
                  (n =
                    (t =
                      (n =
                        (e = parseInt(e.getAttribute("data-start"), 10) || 1) +
                        (a.children.length - 1)) < (t = t < e ? e : t)
                        ? n
                        : t) - e),
                  a.children[n]
                );
            }
          },
          resize: function (e) {
            s([e]);
          },
          assumeViewportIndependence: !0,
        }),
      (t = void 0),
      window.addEventListener("resize", function () {
        (e.assumeViewportIndependence && t === window.innerWidth) ||
          ((t = window.innerWidth),
          s(Array.prototype.slice.call(document.querySelectorAll("pre." + r))));
      }),
      Prism.hooks.add("complete", function (e) {
        var t, n, a;
        e.code &&
          (t = (n = e.element).parentNode) &&
          /pre/i.test(t.nodeName) &&
          (n.querySelector(".line-numbers-rows") ||
            (Prism.util.isActive(n, r) &&
              (n.classList.remove(r),
              t.classList.add(r),
              (n = (n = e.code.match(i)) ? n.length + 1 : 1),
              (n = new Array(n + 1).join("<span></span>")),
              (a = document.createElement("span")).setAttribute(
                "aria-hidden",
                "true",
              ),
              (a.className = "line-numbers-rows"),
              (a.innerHTML = n),
              t.hasAttribute("data-start") &&
                (t.style.counterReset =
                  "linenumber " +
                  (parseInt(t.getAttribute("data-start"), 10) - 1)),
              e.element.appendChild(a),
              s([t]),
              Prism.hooks.run("line-numbers", e))));
      }),
      Prism.hooks.add("line-numbers", function (e) {
        ((e.plugins = e.plugins || {}), (e.plugins.lineNumbers = !0));
      }));
  })(),
  (() => {
    var i, s, o, e, t;
    void 0 !== Prism &&
      "undefined" != typeof document &&
      ((i = []),
      (s = {}),
      (o = function () {}),
      (Prism.plugins.toolbar = {}),
      (e = Prism.plugins.toolbar.registerButton =
        function (e, n) {
          var t =
            "function" == typeof n
              ? n
              : function (e) {
                  var t;
                  return (
                    "function" == typeof n.onClick
                      ? (((t = document.createElement("button")).type =
                          "button"),
                        t.addEventListener("click", function () {
                          n.onClick.call(this, e);
                        }))
                      : "string" == typeof n.url
                        ? ((t = document.createElement("a")).href = n.url)
                        : (t = document.createElement("span")),
                    n.className && t.classList.add(n.className),
                    (t.textContent = n.text),
                    t
                  );
                };
          e in s
            ? console.warn(
                'There is a button with the key "' +
                  e +
                  '" registered already.',
              )
            : i.push((s[e] = t));
        }),
      (t = Prism.plugins.toolbar.hook =
        function (n) {
          var e,
            a,
            t,
            r = n.element.parentNode;
          r &&
            /pre/i.test(r.nodeName) &&
            (r.parentNode.classList.contains("code-toolbar") ||
              ((e = document.createElement("div")).classList.add(
                "code-toolbar",
              ),
              r.parentNode.insertBefore(e, r),
              e.appendChild(r),
              (a = document.createElement("div")).classList.add("toolbar"),
              (r = i),
              (r = (t = ((e) => {
                for (; e; ) {
                  var t = e.getAttribute("data-toolbar-order");
                  if (null != t)
                    return (t = t.trim()).length ? t.split(/\s*,\s*/g) : [];
                  e = e.parentElement;
                }
              })(n.element))
                ? t.map(function (e) {
                    return s[e] || o;
                  })
                : r).forEach(function (e) {
                var t,
                  e = e(n);
                e &&
                  ((t = document.createElement("div")).classList.add(
                    "toolbar-item",
                  ),
                  t.appendChild(e),
                  a.appendChild(t));
              }),
              e.appendChild(a)));
        }),
      e("label", function (e) {
        e = e.element.parentNode;
        if (e && /pre/i.test(e.nodeName) && e.hasAttribute("data-label")) {
          var t,
            n,
            a = e.getAttribute("data-label");
          try {
            n = document.querySelector("template#" + a);
          } catch (e) {}
          return (
            n
              ? (t = n.content)
              : (e.hasAttribute("data-url")
                  ? ((t = document.createElement("a")).href =
                      e.getAttribute("data-url"))
                  : (t = document.createElement("span")),
                (t.textContent = a)),
            t
          );
        }
      }),
      Prism.hooks.add("complete", t));
  })(),
  (() => {
    var n;
    void 0 !== Prism &&
      "undefined" != typeof document &&
      (Prism.plugins.toolbar
        ? ((n = {
            none: "Plain text",
            plain: "Plain text",
            plaintext: "Plain text",
            text: "Plain text",
            txt: "Plain text",
            html: "HTML",
            xml: "XML",
            svg: "SVG",
            mathml: "MathML",
            ssml: "SSML",
            rss: "RSS",
            css: "CSS",
            clike: "C-like",
            js: "JavaScript",
            abap: "ABAP",
            abnf: "ABNF",
            al: "AL",
            antlr4: "ANTLR4",
            g4: "ANTLR4",
            apacheconf: "Apache Configuration",
            apl: "APL",
            aql: "AQL",
            ino: "Arduino",
            arff: "ARFF",
            armasm: "ARM Assembly",
            "arm-asm": "ARM Assembly",
            art: "Arturo",
            asciidoc: "AsciiDoc",
            adoc: "AsciiDoc",
            aspnet: "ASP.NET (C#)",
            asm6502: "6502 Assembly",
            asmatmel: "Atmel AVR Assembly",
            autohotkey: "AutoHotkey",
            autoit: "AutoIt",
            avisynth: "AviSynth",
            avs: "AviSynth",
            "avro-idl": "Avro IDL",
            avdl: "Avro IDL",
            awk: "AWK",
            gawk: "GAWK",
            sh: "Shell",
            basic: "BASIC",
            bbcode: "BBcode",
            bbj: "BBj",
            bnf: "BNF",
            rbnf: "RBNF",
            bqn: "BQN",
            bsl: "BSL (1C:Enterprise)",
            oscript: "OneScript",
            csharp: "C#",
            cs: "C#",
            dotnet: "C#",
            cpp: "C++",
            cfscript: "CFScript",
            cfc: "CFScript",
            cil: "CIL",
            cilkc: "Cilk/C",
            "cilk-c": "Cilk/C",
            cilkcpp: "Cilk/C++",
            "cilk-cpp": "Cilk/C++",
            cilk: "Cilk/C++",
            cmake: "CMake",
            cobol: "COBOL",
            coffee: "CoffeeScript",
            conc: "Concurnas",
            csp: "Content-Security-Policy",
            "css-extras": "CSS Extras",
            csv: "CSV",
            cue: "CUE",
            dataweave: "DataWeave",
            dax: "DAX",
            django: "Django/Jinja2",
            jinja2: "Django/Jinja2",
            "dns-zone-file": "DNS zone file",
            "dns-zone": "DNS zone file",
            dockerfile: "Docker",
            dot: "DOT (Graphviz)",
            gv: "DOT (Graphviz)",
            ebnf: "EBNF",
            editorconfig: "EditorConfig",
            ejs: "EJS",
            etlua: "Embedded Lua templating",
            erb: "ERB",
            "excel-formula": "Excel Formula",
            xlsx: "Excel Formula",
            xls: "Excel Formula",
            fsharp: "F#",
            "firestore-security-rules": "Firestore security rules",
            ftl: "FreeMarker Template Language",
            gml: "GameMaker Language",
            gamemakerlanguage: "GameMaker Language",
            gap: "GAP (CAS)",
            gcode: "G-code",
            gdscript: "GDScript",
            gedcom: "GEDCOM",
            gettext: "gettext",
            po: "gettext",
            glsl: "GLSL",
            gn: "GN",
            gni: "GN",
            "linker-script": "GNU Linker Script",
            ld: "GNU Linker Script",
            "go-module": "Go module",
            "go-mod": "Go module",
            graphql: "GraphQL",
            hbs: "Handlebars",
            hs: "Haskell",
            hcl: "HCL",
            hlsl: "HLSL",
            http: "HTTP",
            hpkp: "HTTP Public-Key-Pins",
            hsts: "HTTP Strict-Transport-Security",
            ichigojam: "IchigoJam",
            "icu-message-format": "ICU Message Format",
            idr: "Idris",
            ignore: ".ignore",
            gitignore: ".gitignore",
            hgignore: ".hgignore",
            npmignore: ".npmignore",
            inform7: "Inform 7",
            javadoc: "JavaDoc",
            javadoclike: "JavaDoc-like",
            javastacktrace: "Java stack trace",
            jq: "JQ",
            jsdoc: "JSDoc",
            "js-extras": "JS Extras",
            json: "JSON",
            webmanifest: "Web App Manifest",
            json5: "JSON5",
            jsonp: "JSONP",
            jsstacktrace: "JS stack trace",
            "js-templates": "JS Templates",
            keepalived: "Keepalived Configure",
            kts: "Kotlin Script",
            kt: "Kotlin",
            kumir: "KuMir (КуМир)",
            kum: "KuMir (КуМир)",
            latex: "LaTeX",
            tex: "TeX",
            context: "ConTeXt",
            lilypond: "LilyPond",
            ly: "LilyPond",
            emacs: "Lisp",
            elisp: "Lisp",
            "emacs-lisp": "Lisp",
            llvm: "LLVM IR",
            log: "Log file",
            lolcode: "LOLCODE",
            magma: "Magma (CAS)",
            md: "Markdown",
            "markup-templating": "Markup templating",
            matlab: "MATLAB",
            maxscript: "MAXScript",
            mel: "MEL",
            metafont: "METAFONT",
            mongodb: "MongoDB",
            moon: "MoonScript",
            n1ql: "N1QL",
            n4js: "N4JS",
            n4jsd: "N4JS",
            "nand2tetris-hdl": "Nand To Tetris HDL",
            naniscript: "Naninovel Script",
            nani: "Naninovel Script",
            nasm: "NASM",
            neon: "NEON",
            nginx: "nginx",
            nsis: "NSIS",
            objectivec: "Objective-C",
            objc: "Objective-C",
            ocaml: "OCaml",
            opencl: "OpenCL",
            openqasm: "OpenQasm",
            qasm: "OpenQasm",
            parigp: "PARI/GP",
            objectpascal: "Object Pascal",
            psl: "PATROL Scripting Language",
            pcaxis: "PC-Axis",
            px: "PC-Axis",
            peoplecode: "PeopleCode",
            pcode: "PeopleCode",
            php: "PHP",
            phpdoc: "PHPDoc",
            "php-extras": "PHP Extras",
            "plant-uml": "PlantUML",
            plantuml: "PlantUML",
            plsql: "PL/SQL",
            powerquery: "PowerQuery",
            pq: "PowerQuery",
            mscript: "PowerQuery",
            powershell: "PowerShell",
            promql: "PromQL",
            properties: ".properties",
            protobuf: "Protocol Buffers",
            purebasic: "PureBasic",
            pbfasm: "PureBasic",
            purs: "PureScript",
            py: "Python",
            qsharp: "Q#",
            qs: "Q#",
            q: "Q (kdb+ database)",
            qml: "QML",
            rkt: "Racket",
            cshtml: "Razor C#",
            razor: "Razor C#",
            jsx: "React JSX",
            tsx: "React TSX",
            renpy: "Ren'py",
            rpy: "Ren'py",
            res: "ReScript",
            rest: "reST (reStructuredText)",
            robotframework: "Robot Framework",
            robot: "Robot Framework",
            rb: "Ruby",
            sas: "SAS",
            sass: "Sass (Sass)",
            scss: "Sass (SCSS)",
            "shell-session": "Shell session",
            "sh-session": "Shell session",
            shellsession: "Shell session",
            sml: "SML",
            smlnj: "SML/NJ",
            solidity: "Solidity (Ethereum)",
            sol: "Solidity (Ethereum)",
            "solution-file": "Solution file",
            sln: "Solution file",
            soy: "Soy (Closure Template)",
            sparql: "SPARQL",
            rq: "SPARQL",
            "splunk-spl": "Splunk SPL",
            sqf: "SQF: Status Quo Function (Arma 3)",
            sql: "SQL",
            stata: "Stata Ado",
            iecst: "Structured Text (IEC 61131-3)",
            supercollider: "SuperCollider",
            sclang: "SuperCollider",
            systemd: "Systemd configuration file",
            "t4-templating": "T4 templating",
            "t4-cs": "T4 Text Templates (C#)",
            t4: "T4 Text Templates (C#)",
            "t4-vb": "T4 Text Templates (VB)",
            tap: "TAP",
            tt2: "Template Toolkit 2",
            toml: "TOML",
            trickle: "trickle",
            troy: "troy",
            trig: "TriG",
            ts: "TypeScript",
            tsconfig: "TSConfig",
            uscript: "UnrealScript",
            uc: "UnrealScript",
            uorazor: "UO Razor Script",
            uri: "URI",
            url: "URL",
            vbnet: "VB.Net",
            vhdl: "VHDL",
            vim: "vim",
            "visual-basic": "Visual Basic",
            vba: "VBA",
            vb: "Visual Basic",
            wasm: "WebAssembly",
            "web-idl": "Web IDL",
            webidl: "Web IDL",
            wgsl: "WGSL",
            wiki: "Wiki markup",
            wolfram: "Wolfram language",
            nb: "Mathematica Notebook",
            wl: "Wolfram language",
            xeoracube: "XeoraCube",
            "xml-doc": "XML doc (.net)",
            xojo: "Xojo (REALbasic)",
            xquery: "XQuery",
            yaml: "YAML",
            yml: "YAML",
            yang: "YANG",
          }),
          Prism.plugins.toolbar.registerButton("show-language", function (e) {
            var t = e.element.parentNode;
            if (t && /pre/i.test(t.nodeName)) {
              e =
                t.getAttribute("data-language") ||
                n[e.language] ||
                ((t = e.language) &&
                  (t.substring(0, 1).toUpperCase() + t.substring(1)).replace(
                    /s(?=cript)/,
                    "S",
                  ));
              if (e)
                return (
                  ((t = document.createElement("span")).textContent = e),
                  t
                );
            }
          }))
        : console.warn("Show Languages plugin loaded before Toolbar plugin."));
  })());
((() => {
  var t = document.querySelectorAll(".kg-audio-card");
  for (let e = 0; e < t.length; e++)
    ((e) => {
      let t = e.querySelector(".kg-audio-player-container"),
        a = e.querySelector(".kg-audio-play-icon"),
        r = e.querySelector(".kg-audio-pause-icon"),
        i = e.querySelector(".kg-audio-seek-slider"),
        o = e.querySelector(".kg-audio-playback-rate"),
        d = e.querySelector(".kg-audio-mute-icon"),
        l = e.querySelector(".kg-audio-unmute-icon"),
        n = e.querySelector(".kg-audio-volume-slider"),
        s = e.querySelector("audio"),
        u = e.querySelector(".kg-audio-duration"),
        c = e.querySelector(".kg-audio-current-time"),
        g = [
          { rate: 0.75, label: "0.7×" },
          { rate: 1, label: "1×" },
          { rate: 1.25, label: "1.2×" },
          { rate: 1.5, label: "1.5×" },
          { rate: 1.75, label: "1.7×" },
          { rate: 2, label: "2×" },
        ],
        v = null,
        k = 1,
        m = () => {
          var e;
          (isNaN(s.duration)
            ? (c.textContent = y(0))
            : ((i.value = Math.floor(s.currentTime)),
              (e = s.duration - s.currentTime),
              (c.textContent = "−" + y(e)),
              t.style.setProperty(
                "--seek-before-width",
                (i.value / i.max) * 100 + "%",
              )),
            (v = requestAnimationFrame(m)));
        },
        p = (e) => {
          e === i
            ? t.style.setProperty(
                "--seek-before-width",
                (e.value / e.max) * 100 + "%",
              )
            : t.style.setProperty(
                "--volume-before-width",
                (e.value / e.max) * 100 + "%",
              );
        },
        y = (e) => {
          var t = Math.floor(e / 60),
            e = Math.floor(e % 60);
          return t + ":" + (e < 10 ? "0" + e : "" + e);
        },
        h = () => {
          u.textContent = y(s.duration);
        },
        L = () => {
          i.max = Math.floor(s.duration);
        },
        f = () => {
          var e;
          0 < s.buffered.length &&
            ((e = Math.floor(s.buffered.end(s.buffered.length - 1))),
            t.style.setProperty("--buffered-width", (e / i.max) * 100 + "%"));
        },
        b =
          (0 < s.readyState
            ? (h(), L(), f(), (c.textContent = y(s.duration)))
            : s.addEventListener("loadedmetadata", () => {
                (h(), L(), f(), (c.textContent = y(s.duration)));
              }),
          e.addEventListener("click", () => {
            (0 < s.currentTime && !s.paused && !s.ended && 2 < s.readyState
              ? q
              : b)();
          }),
          a.addEventListener("click", (e) => {
            (e.stopPropagation(), b());
          }),
          r.addEventListener("click", (e) => {
            (e.stopPropagation(), q());
          }),
          () => {
            (a.classList.add("kg-audio-hide"),
              r.classList.remove("kg-audio-hide"),
              s.play(),
              e.classList.add("kg-audio-is-playing"),
              requestAnimationFrame(m));
          }),
        q = () => {
          (r.classList.add("kg-audio-hide"),
            a.classList.remove("kg-audio-hide"),
            s.pause(),
            e.classList.remove("kg-audio-is-playing"),
            cancelAnimationFrame(v));
        };
      (d.addEventListener("click", () => {
        (event.stopPropagation(),
          d.classList.add("kg-audio-hide"),
          l.classList.remove("kg-audio-hide"),
          (s.muted = !1));
      }),
        l.addEventListener("click", () => {
          (event.stopPropagation(),
            l.classList.add("kg-audio-hide"),
            d.classList.remove("kg-audio-hide"),
            (s.muted = !0));
        }),
        o.addEventListener("click", () => {
          event.stopPropagation();
          var e = g[(k + 1) % g.length];
          ((k += 1), (s.playbackRate = e.rate), (o.textContent = e.label));
        }),
        s.addEventListener("progress", f),
        i.addEventListener("input", (e) => {
          p(e.target);
          e = parseFloat(i.value);
          ((c.textContent = "" + y(e)), s.paused || cancelAnimationFrame(v));
        }),
        s.addEventListener("ended", () => {
          (r.click(), (c.textContent = y(s.duration)));
        }),
        i.addEventListener("change", () => {
          ((s.currentTime = i.value), s.paused || requestAnimationFrame(m));
        }),
        i.addEventListener("click", (e) => {
          e.stopPropagation();
        }),
        n.addEventListener("input", (e) => {
          var t = e.target.value;
          (p(e.target), (s.volume = t / 100));
        }));
    })(t[e]);
})(),
  document.querySelectorAll(".kg-gallery-image img").forEach(function (e) {
    var t = e.closest(".kg-gallery-image"),
      e = e.attributes.width.value / e.attributes.height.value;
    t.style.flex = e + " 1 0%";
  }),
  (() => {
    function t(e) {
      "close" ===
      (e = e.target.closest(".kg-toggle-card")).getAttribute(
        "data-kg-toggle-state",
      )
        ? e.setAttribute("data-kg-toggle-state", "open")
        : e.setAttribute("data-kg-toggle-state", "close");
    }
    var a = document.getElementsByClassName("kg-toggle-heading");
    for (let e = 0; e < a.length; e++) a[e].addEventListener("click", t, !1);
  })(),
  (() => {
    var t = document.querySelectorAll(".kg-video-card");
    for (let e = 0; e < t.length; e++)
      ((e) => {
        e.querySelector(".kg-video-player");
        let t = e.querySelector(".kg-video-player-container"),
          a = e.querySelector(".kg-video-container"),
          r = e.querySelector(".kg-video-play-icon"),
          i = e.querySelector(".kg-video-pause-icon"),
          o = e.querySelector(".kg-video-seek-slider"),
          d = e.querySelector(".kg-video-playback-rate"),
          l = e.querySelector(".kg-video-mute-icon"),
          n = e.querySelector(".kg-video-unmute-icon"),
          s = e.querySelector(".kg-video-volume-slider"),
          u = e.querySelector("video"),
          c = e.querySelector(".kg-video-duration"),
          g = e.querySelector(".kg-video-current-time"),
          v = e.querySelector(".kg-video-large-play-icon"),
          k = e.querySelector(".kg-video-overlay"),
          m = [
            { rate: 0.75, label: "0.7×" },
            { rate: 1, label: "1×" },
            { rate: 1.25, label: "1.2×" },
            { rate: 1.5, label: "1.5×" },
            { rate: 1.75, label: "1.7×" },
            { rate: 2, label: "2×" },
          ],
          p = null,
          y = 1,
          h =
            (u.loop && k.classList.add("kg-video-hide-animated"),
            () => {
              var e;
              (isNaN(u.duration)
                ? (g.textContent = f(0))
                : ((o.value = Math.floor(u.currentTime)),
                  (e = u.duration - u.currentTime),
                  (g.textContent = "−" + f(e)),
                  t.style.setProperty(
                    "--seek-before-width",
                    (o.value / o.max) * 100 + "%",
                  )),
                (p = requestAnimationFrame(h)));
            }),
          L = (e) => {
            e === o
              ? t.style.setProperty(
                  "--seek-before-width",
                  (e.value / e.max) * 100 + "%",
                )
              : t.style.setProperty(
                  "--volume-before-width",
                  (e.value / e.max) * 100 + "%",
                );
          },
          f = (e) => {
            var t = Math.floor(e / 60),
              e = Math.floor(e % 60);
            return t + ":" + (e < 10 ? "0" + e : "" + e);
          },
          b = () => {
            c.textContent = f(u.duration);
          },
          q = () => {
            o.max = Math.floor(u.duration);
          },
          S = () => {
            var e;
            0 < u.buffered.length &&
              ((e = Math.floor(u.buffered.end(u.buffered.length - 1))),
              t.style.setProperty("--buffered-width", (e / o.max) * 100 + "%"));
          },
          E =
            (0 < u.readyState
              ? (b(),
                q(),
                S(),
                (g.textContent = f(u.duration)),
                u.autoplay &&
                  ((p = requestAnimationFrame(h)),
                  r.classList.add("kg-video-hide"),
                  i.classList.remove("kg-video-hide")),
                u.muted &&
                  (n.classList.add("kg-video-hide"),
                  l.classList.remove("kg-video-hide")))
              : u.addEventListener("loadedmetadata", () => {
                  (b(),
                    q(),
                    S(),
                    (g.textContent = f(u.duration)),
                    u.autoplay &&
                      ((p = requestAnimationFrame(h)),
                      r.classList.add("kg-video-hide"),
                      i.classList.remove("kg-video-hide")),
                    u.muted &&
                      (n.classList.add("kg-video-hide"),
                      l.classList.remove("kg-video-hide")));
                }),
            k.addEventListener("click", () => {
              E();
            }),
            e.addEventListener("click", () => {
              u.loop ||
                (0 < u.currentTime && !u.paused && !u.ended && 2 < u.readyState
                  ? x
                  : E)();
            }),
            (u.onplay = () => {
              (a.classList.add("kg-video-has-played"),
                r.classList.add("kg-video-hide"),
                i.classList.remove("kg-video-hide"));
            }),
            () => {
              (a.classList.add("kg-video-is-playing"),
                r.classList.add("kg-video-hide"),
                i.classList.remove("kg-video-hide"),
                u.play(),
                (p = requestAnimationFrame(h)));
            }),
          x = () => {
            (a.classList.remove("kg-video-is-playing"),
              i.classList.add("kg-video-hide"),
              r.classList.remove("kg-video-hide"),
              u.pause(),
              cancelAnimationFrame(p));
          };
        (v.addEventListener("click", (e) => {
          (e.stopPropagation(), E());
        }),
          r.addEventListener("click", (e) => {
            (e.stopPropagation(), E());
          }),
          i.addEventListener("click", (e) => {
            (e.stopPropagation(), x());
          }),
          l.addEventListener("click", (e) => {
            (e.stopPropagation(),
              l.classList.add("kg-video-hide"),
              n.classList.remove("kg-video-hide"),
              (u.muted = !1));
          }),
          n.addEventListener("click", (e) => {
            (e.stopPropagation(),
              n.classList.add("kg-video-hide"),
              l.classList.remove("kg-video-hide"),
              (u.muted = !0));
          }),
          d.addEventListener("click", (e) => {
            e.stopPropagation();
            e = m[(y + 1) % m.length];
            ((y += 1), (u.playbackRate = e.rate), (d.textContent = e.label));
          }),
          u.addEventListener("progress", S),
          o.addEventListener("input", (e) => {
            (e.stopPropagation(), L(e.target));
            e = parseFloat(o.value);
            ((g.textContent = "" + f(e)), u.paused || cancelAnimationFrame(p));
          }),
          o.addEventListener("change", (e) => {
            (e.stopPropagation(),
              (u.currentTime = o.value),
              u.paused || requestAnimationFrame(h));
          }),
          o.addEventListener("click", (e) => {
            e.stopPropagation();
          }),
          s.addEventListener("input", (e) => {
            e.stopPropagation();
            var t = e.target.value;
            (L(e.target), (u.volume = t / 100));
          }),
          u.addEventListener("ended", () => {
            (i.click(), (g.textContent = f(u.duration)));
          }));
      })(t[e]);
  })());
let shuffle = function (e) {
    let t = e.length;
    for (var o, i; 0 !== t; )
      ((i = Math.floor(Math.random() * t)),
        (o = e[--t]),
        (e[t] = e[i]),
        (e[i] = o));
    return e;
  },
  initTime = () => {
    var e = document.querySelector(".js-time");
    e && updateTime(e);
  },
  updateTime = function (e) {
    var t = luxon.DateTime.now()
      .setZone("America/Los_Angeles")
      .toFormat("H:mm:ss");
    ((e.textContent = t),
      setTimeout(function () {
        updateTime(e);
      }, 1e3));
  },
  initExternalLinks = () => {
    var o = document.links;
    for (let e = 0, t = o.length; e < t; e++)
      o[e].hostname != window.location.hostname &&
        ((o[e].target = "_blank"), (o[e].rel = "noopener"));
  },
  initSwapQuote = () => {
    let t = document.querySelector(".js-quote"),
      o = document.querySelector(".js-quote-attribution");
    var i = document.querySelector(".js-quote-button");
    if (t && i) {
      let e = [
        {
          quote:
            "Some people, they move the piano. Some people, they play the piano.",
          attribution: "Oscar Schmidt",
        },
        {
          quote: "A good plain look is my favorite look.",
          attribution: "Andy Warhol",
        },
        {
          quote:
            "You can never invite the wind, but you must leave the window open.",
          attribution: "Bruce Lee",
        },
        {
          quote: "“What am I really sick of?” is where innovation begins.",
          attribution: "Jerry Seinfeld",
        },
        {
          quote: "To learn business, do business.",
          attribution: "Jason Fried",
        },
        {
          quote:
            "No creative person I know has ever asked for a brainstorming session.",
          attribution: "Brian Collins",
        },
        {
          quote:
            "People like you more when you are working towards something, not when you have it.",
          attribution: "Drake",
        },
        {
          quote:
            "Technology challenges us to assert our human values which means that first of all, we need to figure out what they are.",
          attribution: "Sherry Turkle",
        },
        { quote: "Curiosity is the best guide.", attribution: "Paul Graham" },
        {
          quote:
            "When you’re always looking over your peers’ shoulders, you end up looking like your peers.",
          attribution: "Brian Collins",
        },
        {
          quote:
            "Don’t demand that things happen as you wish, but wish that they happen as they do happen, and you will go on well.",
          attribution: "Epictetus",
        },
        {
          quote: "If an idea is good, the work flows easily.",
          attribution: "Morgan Housel",
        },
        {
          quote:
            "It’s not that people want to ride a bicycle for a bicycle’s sake, it’s that they want to go from Point A to Point B.",
          attribution: "Ben Thompson",
        },
        {
          quote:
            "Don’t let yourself get drawn into chasing something just because others are.",
          attribution: "Paul Graham",
        },
        {
          quote: "Show me the incentive and I’ll show you the outcome.",
          attribution: "Charlie Munger",
        },
        {
          quote:
            "When a horse wants to run there ain’t no sense in closing the gate.",
          attribution: "“Space Cowboy” by Kacey Musgraves",
        },
        {
          quote:
            "Like a palm tree in the wind, I won’t break, I’ll just bend. And I’ll sway.",
          attribution: "“Sway” by Kacey Musgraves",
        },
        {
          quote: "The new needs friends.",
          attribution: "Anton Ego, <em>Ratatouille</em>",
        },
        {
          quote:
            "Not everyone can become a great artist, but a great artist can come from anywhere.",
          attribution: "Anton Ego, <em>Ratatouille</em>",
        },
        {
          quote: "Nature does not hurry, yet everything is accomplished.",
          attribution: "Laozi",
        },
        {
          quote: "You can’t stop the waves, but you can learn to surf.",
          attribution: "Jon Kabat-Zinn",
        },
        { quote: "We are what we repeatedly do.", attribution: "Aristotle" },
        { quote: "Measure twice, cut once.", attribution: "Proverb" },
        { quote: "Haste makes waste.", attribution: "Proverb" },
        { quote: "Look before you leap.", attribution: "Proverb" },
        { quote: "A stitch in time saves nine.", attribution: "Proverb" },
        { quote: "You are what you eat.", attribution: "Proverb" },
        { quote: "First, do no harm.", attribution: "Latin Proverb" },
        {
          quote: "There is no accounting for taste.",
          attribution: "Latin Proverb",
        },
        {
          quote: "Life finds a way.",
          attribution: "Dr. Ian Malcolm, <em>Jurassic Park</em>",
        },
        {
          quote:
            "Your scientists were so preoccupied with whether or not they could, they didn’t stop to think if they should.",
          attribution: "Dr. Ian Malcolm, <em>Jurassic Park</em>",
        },
        {
          quote: "If I’d observed all the rules, I’d never have got anywhere.",
          attribution: "Marilyn Monroe",
        },
        {
          quote: "Good design makes a product understandable.",
          attribution: "Dieter Rams",
        },
        {
          quote: "Brevity is the soul of wit.",
          attribution: "William Shakespeare",
        },
        {
          quote:
            "We’re all different. But there’s something kind of fantastic about that, isn’t there?",
          attribution: "Mrs. Felicity Fox, <em>Fantastic Mr. Fox</em>",
        },
        {
          quote:
            "Do what you believe in. But it’s not easy when you walk your own road. You’ve only got yourself to blame.",
          attribution: "Seiya Tsukushima, <em>Whisper of the Heart</em>",
        },
        {
          quote: "Somebody help me, I’m being spontaneous!",
          attribution: "Truman Burbank, <em>The Truman Show</em>",
        },
        {
          quote: "It isn’t always Shakespeare, but it’s genuine.",
          attribution: "Christof, <em>The Truman Show</em>",
        },
        {
          quote: "The fool looks at a finger that points at the sky.",
          attribution: "The Sacré-Cœur boy, <em>Amelie</em>",
        },
        {
          quote:
            "Make every detail perfect and limit the number of details to perfect.",
          attribution: "Jack Dorsey",
        },
        {
          quote:
            "The only way I can get you to do anything is by giving you what you want.",
          attribution: "Dale Carnegie",
        },
        {
          quote: "To achieve style, begin by affecting none.",
          attribution: "E.B. White, <em>The Elements of Style</em>",
        },
        {
          quote: "Do not take shortcuts at the cost of clarity.",
          attribution: "E.B. White, <em>The Elements of Style</em>",
        },
        {
          quote: "Choose a suitable design and hold to it.",
          attribution: "E.B. White, <em>The Elements of Style</em>",
        },
        {
          quote:
            "If you concentrate on the rear mirror, you’ll crash and cause an accident.",
          attribution: "Eliud Kipchoge",
        },
        { quote: "Run with a relaxed mind.", attribution: "Eliud Kipchoge" },
        {
          quote:
            "Unburdened by professionalism or legacy, [the work of young creatives] is uninhibited and, therefore, mesmerizing.",
          attribution: "Brian Collins",
        },
        {
          quote:
            "Everything should be made as simple as possible, but no simpler.",
          attribution: "Albert Einstein",
        },
        {
          quote:
            "I didn’t intend for ten-year-old me to come on this trip. But somehow, once she showed up, she wouldn’t leave me alone.",
          attribution: "Taeko Okajima, <em>Only Yesterday</em>",
        },
        {
          quote:
            "Without even thinking about it, I used to be able to fly. Now I’m trying to look inside myself and find out how I did it.",
          attribution: "Kiki, <em>Kiki's Delivery Service</em>",
        },
        { quote: "Straightforward is best.", attribution: "" },
        { quote: "Constraints help creativity.", attribution: "" },
        { quote: "Critical thinking is invaluable.", attribution: "" },
        { quote: "People respect honesty.", attribution: "" },
        { quote: "Process matters.", attribution: "" },
        { quote: "Consistency is underrated.", attribution: "" },
        { quote: "Humor shouldn’t be forgotten.", attribution: "" },
        { quote: "New is not often an improvement.", attribution: "" },
        {
          quote: "Sometimes “nothing” is better than “something.”",
          attribution: "",
        },
        { quote: "When in doubt, go with black.", attribution: "" },
        {
          quote: "Propose honesty as a solution.",
          attribution: "Daniel Eatock",
        },
        {
          quote: "Make something difficult look easy.",
          attribution: "Daniel Eatock",
        },
        { quote: "Subvert expectation.", attribution: "Daniel Eatock" },
        {
          quote:
            "The most regretful people on earth are those who felt the call to creative work […] and gave to it neither power nor time.",
          attribution: "Mary Oliver",
        },
        {
          quote:
            "Instructions for living a life: Pay attention. Be astonished. Tell about it.",
          attribution: "Mary Oliver",
        },
        {
          quote:
            "Working on an unfashionable problem can be very pleasing. There’s no hype or hurry. Opportunists and critics are occupied elsewhere.",
          attribution: "Paul Graham",
        },
        {
          quote:
            "Not that the story need be long, but it will take a long while to make it short.",
          attribution: "Henry David Thoreau",
        },
        {
          quote: "There are always flowers for those who want to see them.",
          attribution: "Henri Matisse",
        },
        {
          quote: "The best style is the one you can’t help but have.",
          attribution: "",
        },
        {
          quote: "Good things are always good.",
          attribution: "Natsuki Obana, <em>La Grande Maison Tokyo</em>",
        },
        {
          quote:
            "Just keep taking out words, and if it still makes sense, go with it.",
          attribution: "",
        },
        {
          quote:
            "How much pain they have cost us, the evils which have never happened.",
          attribution: "Thomas Jefferson",
        },
        {
          quote:
            "It’s hard to remember how you felt when you know how the story ends.",
          attribution: "Morgan Housel",
        },
        {
          quote:
            "We were not supposed to hate Mondays and live for the weekends and holidays.",
          attribution: "Charles Eisenstien",
        },
        {
          quote:
            "Solitude will cure our distaste for a crowd, and a crowd will cure our boredom with solitude.",
          attribution: "Seneca",
        },
        {
          quote: "We have, and we have always had, all the time there is.",
          attribution: "Arnold Bennett",
        },
        {
          quote: "The loudest boos come from the cheapest seats.",
          attribution: "Babe Ruth",
        },
        {
          quote: "If you’re paying attention, the whole world is a classroom.",
          attribution: "Jeff Bezos",
        },
        {
          quote: "Don’t use seven words when four will do.",
          attribution: "Rusty Ryan, <em>Ocean’s Eleven</em>",
        },
        {
          quote: "Look always at your mark but don’t stare.",
          attribution: "Rusty Ryan, <em>Ocean’s Eleven</em>",
        },
        {
          quote: "One must imagine Sisyphus happy.",
          attribution: "Albert Camus",
        },
        {
          quote:
            "And if the System was going to chase me to the ends of the earth, when would I find the time to memorize irregular Greek verbs?",
          attribution:
            "<em>Hard-Boiled Wonderland and the End of the World</em> by Haruki Murakami",
        },
        {
          quote:
            "Mama said there’ll be days like this, there’ll be days like this, Mama said.",
          attribution:
            "Song by Luther Dixon and Willie Denson, recorded by the Shirelles",
        },
        { quote: "It’s just design.", attribution: "" },
        { quote: "Work, like love, finds a way.", attribution: "Paul Graham" },
        { quote: "Attention is a zero-sum game.", attribution: "Paul Graham" },
        {
          quote: "Simplicity, wit, and good typography.",
          attribution: "Michael Bierut",
        },
        {
          quote:
            "It’s amazing when people tell me that electronic music has not got soul. You can’t blame the computer. If there’s not soul in the music, it’s because nobody put it there.",
          attribution: "",
        },
        {
          quote:
            "There is only one success: To be able to spend your life in your own way.",
          attribution: "Christopher Morley",
        },
        { quote: "Out of the work comes the work.", attribution: "John Cage" },
        {
          quote: "Hunger is the best appetizer.",
          attribution: "Attributed to Diogenes",
        },
        {
          quote: "Truly love the people with whom destiny has surrounded you.",
          attribution: "Marcus Aurelius",
        },
        {
          quote: "How you do anything is how you do everything.",
          attribution: "Derek Sivers",
        },
        {
          quote:
            "You get no competitive advantage from consuming the same stuff everyone else is consuming.",
          attribution: "Derek Sivers",
        },
        {
          quote: "If we’re not surprised, we’re not learning.",
          attribution: "Derek Sivers",
        },
        {
          quote: "Resist the urge to figure it all out in advance.",
          attribution: "Derek Sivers",
        },
        { quote: "These are the good old days.", attribution: "" },
        { quote: "Anima sana in corpore sano.", attribution: "" },
        {
          quote: "Things are simple until you make them complicated.",
          attribution: "",
        },
        {
          quote: "A little less conversation, a little more action.",
          attribution:
            "Song by Mac Davis and Billy Strange, recorded by Elvis Presley",
        },
        {
          quote: "You can’t hurry love.",
          attribution:
            "Song by Holland–Dozier–Holland, recorded by the Supremes",
        },
        {
          quote:
            "Everything around you that you call life was made up by people that were no smarter than you.",
          attribution: "Steve Jobs",
        },
        {
          quote: "I’m as proud of what we don’t do as I am of what we do.",
          attribution: "Steve Jobs",
        },
        {
          quote: "If the work is unfulfilling, the money will be too.",
          attribution: "Jerry Seinfeld",
        },
        {
          quote:
            "The people with the interesting answers are those who ask the interesting questions.",
          attribution: "David Bayles and Ted Orland, <em>Art & Fear</em>",
        },
        {
          quote: "Writing by implication should be one of your goals.",
          attribution:
            "Verlyn Klinkenborg, <em>Several Short Sentences About Writing</em>",
        },
        {
          quote: "Imagine a reader you can trust.",
          attribution:
            "Verlyn Klinkenborg, <em>Several Short Sentences About Writing</em>",
        },
        {
          quote: "The work selects its audience.",
          attribution:
            "Verlyn Klinkenborg, <em>Several Short Sentences About Writing</em>",
        },
        {
          quote: "The unconscious mind needs to work.",
          attribution: "Jack Ellis",
        },
        {
          quote:
            "Did you feel you were tricked, by the future you picked? Well, come on down.",
          attribution: "“Down to Earth” by Peter Gabriel",
        },
        {
          quote: "When everyone’s super, no one will be.",
          attribution: "Syndrome, <em>The Incredibles</em>",
        },
        {
          quote: "Quality never goes out of style.",
          attribution: "Levi Strauss & Co.",
        },
        {
          quote: "Regret is the proof that love was there once.",
          attribution: "<em>Going My Home</em>",
        },
        {
          quote:
            "This world isn’t made up with just what you can see with your eyes.",
          attribution: "<em>Going My Home</em>",
        },
        {
          quote:
            "If you don’t look for them, you won’t know if they are there or not.",
          attribution: "Osamu Torii, <em>Going My Home</em>",
        },
        {
          quote:
            "It’s those things you just glance at that can be what’s important.",
          attribution: "Sae Tsuboi, <em>Going My Home</em>",
        },
        {
          quote: "Do things for their own sake.",
          attribution: "Naval Ravikant",
        },
        {
          quote: "No one can compete with you on being you.",
          attribution: "Naval Ravikant",
        },
        {
          quote: "The most personal is the most creative.",
          attribution: "Bong Joon-ho",
        },
        {
          quote:
            "We all get confronted with how little we know from time to time.",
          attribution: "Madoka Yoshiyama, <em>He Who Can’t Marry</em>",
        },
        {
          quote: "First make the change easy, then make the easy change.",
          attribution: "",
        },
        {
          quote: "It can be half, but not half-assed.",
          attribution: "Jason Fried",
        },
        {
          quote: "Baseball becomes really hard when it’s not fun.",
          attribution: "John Jaso",
        },
        {
          quote: "Take a simple, basic idea and take it very seriously.",
          attribution: "Charlie Munger",
        },
        {
          quote: "Mimicking the herd invites regression to the mean.",
          attribution: "Charlie Munger",
        },
        {
          quote: "Remember that you are the easiest person to fool.",
          attribution: "Charlie Munger",
        },
        {
          quote: "Keep things simple and remember what you set out to do.",
          attribution: "Charlie Munger",
        },
        {
          quote: "You can’t go fishing without bait.",
          attribution: "Greg Maddux",
        },
        {
          quote: "Weird is different, but different isn’t necessarily weird.",
          attribution: "",
        },
        { quote: "Short and sweet, like a haiku.", attribution: "" },
        {
          quote: "Champions behave like champions before they’re champions.",
          attribution: "Bill Walsh",
        },
        {
          quote: "Any new and better way of doing things is technology.",
          attribution: "Peter Thiel",
        },
        { quote: "We forget that we forget.", attribution: "" },
        {
          quote: "One kind of rice raises hundreds of kinds of people.",
          attribution: "",
        },
        {
          quote: "The world is not driven by greed. It’s driven by envy.",
          attribution: "Charlie Munger",
        },
        {
          quote:
            "For the highest levels to be attainable over time, the training process has to be sustainable.",
          attribution: "Stephen Seiler",
        },
        {
          quote: "Convention became conventional because it works.",
          attribution: "Stewart Brand, <em>How Buildings Learn</em>",
        },
        {
          quote:
            "The most extreme mistakes are mistakes of omission. They do not show up in our figures. they show up in our opportunity costs.",
          attribution: "Charlie Munger",
        },
        {
          quote:
            "All of my best decisions in business and in life have been made with heart, intuition, guts, not analysis.",
          attribution: "Jeff Bezos",
        },
        {
          quote: "I believe in the power of wandering.",
          attribution: "Jeff Bezos",
        },
        {
          quote:
            "We’re not interested in being the best. We’re interested in being the only.",
          attribution: "Brian Collins",
        },
        {
          quote:
            "I’m always trying to turn things upside down to see if they look any better.",
          attribution: "Tibor Kalman",
        },
        {
          quote:
            "Go as far as you can see. Once you get there, you’ll be able to see further.",
          attribution: "Jonathan Neman",
        },
        {
          quote: "Clarifying is our business, obscuring is our pleasure.",
          attribution: "Sulki and Min",
        },
        {
          quote:
            "Facts are interesting, but imagination creates possibilities.",
          attribution: "Brian Collins",
        },
        {
          quote:
            "The important thing we do as managers is to find .400 hitters and then not tell them how to swing.",
          attribution: "Warren Buffett",
        },
        {
          quote:
            "Waking in the morning / Time smiles in my hand. / This dawn / lasts all day.",
          attribution: "Deena Metzger",
        },
        { quote: "You are enough.", attribution: "" },
        {
          quote:
            "All my practices, honestly, were dedicated to be better. I never approach a single practice without the motivation to improve something.",
          attribution: "Rafael Nadal",
        },
        {
          quote:
            "If walking on the moon left astronauts underwhelmed, what does it say about our own earthly goals and expectations?",
          attribution: "Morgan Housel",
        },
        {
          quote:
            "Getting the same few things right in different ways is a career’s worth of work.",
          attribution: "Jason Fried",
        },
        { quote: "History holds forgotten solutions.", attribution: "" },
        { quote: "Slow is smooth, smooth is fast.", attribution: "" },
        { quote: "Time will tell.", attribution: "" },
        {
          quote: "Not groundbreaking. Just grounded.",
          attribution: "Jason Fried",
        },
      ];
      ((e = shuffle(e)),
        (t.innerHTML = e[0].quote),
        (o.innerHTML = e[0].attribution ? " —&hairsp;" + e[0].attribution : ""),
        i.addEventListener("click", function () {
          (e.push(e.splice(0, 1)[0]),
            (t.innerHTML = e[0].quote),
            (o.innerHTML = e[0].attribution
              ? " —&hairsp;" + e[0].attribution
              : ""));
        }));
    }
  },
  initRecentlyPlayed = () => {
    let n = document.querySelector(".js-recently-played"),
      r = document.querySelector(".js-recently-played-time");
    if (n && r)
      fetch(
        "https://ws.audioscrobbler.com/2.0/?method=user.getrecenttracks&user=justinjaywang&api_key=549811d11e16f91bfe44859a356abad5&format=json&limit=1",
      )
        .then((e) => e.json())
        .then((e) => {
          var e = e.recenttracks.track[0],
            t = !(!e["@attr"] || !e["@attr"].nowplaying),
            o = new Date(),
            i = new Date(e.date ? 1e3 * e.date.uts : o);
          let a;
          ((a = t
            ? "Just now"
            : (t = Math.floor((o - i) / 1e3)) < 60
              ? t + "s ago"
              : t < 3600
                ? Math.floor(t / 60) + "m ago"
                : t < 86400
                  ? Math.floor(t / 3600) + "h ago"
                  : Math.floor(t / 86400) + "d ago"),
            (n.innerHTML = `“${e.name.split(" - ")[0]}” by ${e.artist["#text"]} `),
            (r.innerHTML = "" + a));
        })
        .catch((e) => {
          console.error("Error fetching recently played track:", e);
        });
  },
  handleForm = function (e) {
    let i = document.querySelector(".js-form-" + e),
      a = document.querySelector(`.js-form-${e}-content`),
      n = document.querySelector(`.js-form-${e}-success`),
      r = document.querySelector(`.js-form-${e}-blurb`);
    if (i && a && n && r) {
      i.addEventListener("submit", async function (e) {
        e.preventDefault();
        var t = window.location.href,
          o = document.createElement("input"),
          t =
            ((o.type = "hidden"),
            (o.name = "url"),
            (o.value = t),
            i.querySelector("input[name='url']") || i.appendChild(o),
            new FormData(e.target));
        fetch(e.target.action, {
          method: i.method,
          body: t,
          headers: { Accept: "application/json" },
        })
          .then((e) => {
            e.ok
              ? (n.classList.remove("hidden"),
                a.classList.add("hidden"),
                i.reset())
              : e.json().then((e) => {
                  Object.hasOwn(e, "errors")
                    ? ((n.innerHTML = e.errors
                        .map((e) => e.message)
                        .join(", ")),
                      n.classList.remove("hidden"))
                    : (r.innerHTML = r.dataset.error);
                });
          })
          .catch((e) => {
            r.innerHTML = r.dataset.error;
          });
      });
      let e = i.querySelector("input"),
        t = i.querySelector("[type='submit']"),
        o = t.dataset.disabledFade;
      e.addEventListener("input", function () {
        e.value.trim()
          ? (t.classList.add("text-primary"),
            t.classList.remove("text-primary/" + o),
            t.removeAttribute("disabled"))
          : (t.classList.remove("text-primary"),
            t.classList.add("text-primary/" + o),
            t.setAttribute("disabled", "disabled"));
      });
    }
  },
  initHandleGuestbook = () => {
    handleForm("guestbook");
  },
  initHandleSubscribe = () => {
    handleForm("subscribe");
  },
  initShowcase = () => {
    let i = document.querySelectorAll(".js-showcase-item"),
      e = 0,
      o = 3;
    setInterval(() => {
      var o;
      ((e = (e + 1) % i.length),
        (o = e),
        i.forEach((e, t) => {
          t === o
            ? (e.classList.remove("hidden"), e.classList.add("block"))
            : (e.classList.remove("block"), e.classList.add("hidden"));
        }));
    }, 1500);
    let a = setInterval(() => {
      var e, t;
      o < i.length
        ? ((e = i[o]),
          (t = e.querySelector("img")),
          (e = e.querySelector("source")),
          (t.src = t.dataset.src),
          t.removeAttribute("data-src"),
          (t.srcset = t.dataset.srcset),
          t.removeAttribute("data-srcset"),
          (e.srcset = e.dataset.srcset),
          e.removeAttribute("data-srcset"),
          o++)
        : clearInterval(a);
    }, 1500);
  },
  initCoverImage = () => {
    var e = document.querySelector(".js-cover");
    let t = document.querySelector(".js-cover-scrim");
    e &&
      t &&
      (e.complete && 0 !== e.naturalWidth
        ? t.classList.remove("hidden")
        : (e.onload = function () {
            t.classList.remove("hidden");
          }));
  },
  initImageWrap = () => {
    document.querySelectorAll(".rich-text").forEach((e) => {
      e.querySelectorAll("img, video").forEach((e) => {
        var t, o;
        e.getAttribute("src") &&
          "" !== e.getAttribute("src").trim() &&
          ((t = document.createElement("div")).classList.add("media-wrap"),
          (o = [...e.classList]),
          t.classList.add(...o),
          (e.className = ""),
          e.parentNode.insertBefore(t, e),
          t.appendChild(e));
      });
    });
  },
  initBookmarkCard = () => {
    document.querySelectorAll(".kg-bookmark-card").forEach((e) => {
      var t = e.querySelector(".kg-bookmark-container"),
        t = new URL(t.href).hostname.replace("www.", ""),
        e = e.querySelector(".kg-bookmark-metadata");
      e && e.append(document.createTextNode(t));
    });
  },
  initFilterAndSortTagIndices = () => {
    let t = document.querySelector(".js-tags");
    if (t) {
      var e = Array.from(document.querySelectorAll("[data-tag-slug]"));
      let i = new Set();
      e = e
        .reduce((e, t) => {
          var o = t.dataset.tagSlug;
          return (i.has(o) || (i.add(o), e.push(t)), e);
        }, [])
        .sort((e, t) => {
          e = parseInt(
            e.querySelector("[data-tag-count]").dataset.tagCount,
            10,
          );
          return (
            parseInt(t.querySelector("[data-tag-count]").dataset.tagCount, 10) -
            e
          );
        });
      ((t.innerHTML = ""), e.forEach((e) => t.appendChild(e)));
    }
  },
  initPaginationDropdown = () => {
    let a = document.getElementById("pagination-dropdown");
    if (a) {
      var e = parseInt(a.dataset.total, 10),
        t = parseInt(a.dataset.current, 10);
      let i = a.querySelector("option").dataset.url;
      (Array.from({ length: e - 1 }, (e, t) => {
        var t = t + 2,
          o = new Option(t, t);
        ((o.dataset.url = i + `page/${t}/`), (a.options[t - 1] = o));
      }),
        (a.value = t),
        a.addEventListener("change", function () {
          window.location.href = this.options[this.selectedIndex].dataset.url;
        }));
    }
  },
  initPaginationKeys = () => {
    let t = document.getElementById("pagination-prev"),
      o = document.getElementById("pagination-next");
    (t || o) &&
      document.addEventListener("keydown", (e) => {
        "INPUT" !== e.target.tagName &&
          "TEXTAREA" !== e.target.tagName &&
          ("ArrowLeft" === e.key && t
            ? (window.location.href = t.href)
            : "ArrowRight" === e.key && o && (window.location.href = o.href));
      });
  },
  initPhotoArchiveYears = () => {
    let n = document.querySelector(".js-photo-archive");
    if (n) {
      var e = n.querySelectorAll(".js-photo-archive-item");
      let a = null;
      e.forEach((e) => {
        var t,
          o,
          i = e.dataset.year;
        i &&
          i !== a &&
          (((t = document.createElement("div")).className =
            "col-span-12 md:col-span-8 xl:col-span-6"),
          ((o = document.createElement("h3")).className =
            "aspect-square rounded bg-primary text-background font-bold text-lg flex items-center justify-center"),
          (o.textContent = i),
          t.appendChild(o),
          n.insertBefore(t, e),
          (a = i));
      });
    }
  },
  foldGallery = function (e) {
    let t = Array.from(e.children),
      i = t[0].querySelector("figcaption"),
      o =
        (t.forEach((e, t) => {
          0 < t &&
            i &&
            ((o = i.cloneNode(!0)),
            e.querySelector("figcaption") || e.appendChild(o));
          var o = e.querySelector("img");
          o &&
            (o.removeAttribute("loading"),
            t < 3
              ? ((o.src = o.dataset.src || o.src),
                (o.srcset = o.dataset.srcset || o.srcset))
              : (o.src && ((o.dataset.src = o.src), o.removeAttribute("src")),
                o.srcset &&
                  ((o.dataset.srcset = o.srcset),
                  o.removeAttribute("srcset"))));
        }),
        (i) => {
          t.forEach((e, t) => {
            var o = e.querySelector("img");
            o &&
              (e.classList.toggle("block", t === i),
              e.classList.toggle("hidden", t !== i),
              i <= t) &&
              t < i + 3 &&
              (o.dataset.src && !o.src && (o.src = o.dataset.src),
              o.dataset.srcset) &&
              !o.srcset &&
              (o.srcset = o.dataset.srcset);
          });
        }),
      a = 0;
    (o(a),
      (e.next = () => {
        ((a = (a + 1) % t.length), o(a));
      }));
  },
  initTriggerGalleries = function () {
    document.querySelectorAll(".js-gallery-trigger").forEach((e) => {
      var t = e.dataset.target;
      let o = document.querySelector(`.js-gallery[data-id="${t}"]`);
      o &&
        (foldGallery(o),
        e.addEventListener("click", (e) => {
          (e.preventDefault(), o.next());
        }),
        e.classList.remove("hidden"),
        o.classList.remove("hidden"));
    });
  },
  initAutoGalleries = function () {
    document.querySelectorAll(".js-auto-gallery").forEach((e) => {
      foldGallery(e);
      var t = parseInt(e.dataset.interval) || 1500;
      (setInterval(() => e.next(), t), e.classList.remove("hidden"));
    });
  };
document.addEventListener("DOMContentLoaded", function () {
  (initTime(),
    initShowcase(),
    initCoverImage(),
    initRecentlyPlayed(),
    initHandleGuestbook(),
    initHandleSubscribe(),
    initExternalLinks(),
    initImageWrap(),
    initBookmarkCard(),
    initSwapQuote(),
    initFilterAndSortTagIndices(),
    initPaginationDropdown(),
    initPaginationKeys(),
    initPhotoArchiveYears(),
    initTriggerGalleries(),
    initAutoGalleries());
});
